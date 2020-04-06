use async_postgres::TlsConfig;
use std::env;
use std::error::Error;
use std::fs;
use std::io;

const TEST_TLS_URL: &str = env!("TEST_TLS_URL");

#[async_std::test]
async fn query() -> Result<(), Box<dyn Error>> {
    let mut cert_buf = io::BufReader::new(fs::File::open("tests/cert/server.crt")?);
    let mut tls_config = TlsConfig::new();
    tls_config.root_store.add_pem_file(&mut cert_buf).unwrap();
    let (client, conn) =
        async_postgres::connect_with(&TEST_TLS_URL.parse()?, tls_config).await?;
    async_std::task::spawn(conn);
    let row = client
        .query_one("SELECT * FROM posts WHERE id=$1", &[&1i32])
        .await?;
    assert_eq!("MIT LICENSE", row.get::<_, &str>(1));
    assert_eq!(
        "Permission is hereby granted, free of charge, to any\nperson obtaining a copy of this software and associated\ndocumentation files (the \"Software\"), to deal in the\nSoftware without restriction, including without\nlimitation the rights to use, copy, modify, merge,\npublish, distribute, sublicense, and/or sell copies of\nthe Software, and to permit persons to whom the Software\nis furnished to do so, subject to the following\nconditions:\n\nThe above copyright notice and this permission notice\nshall be included in all copies or substantial portions\nof the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF\nANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED\nTO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A\nPARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT\nSHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\nCLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION\nOF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR\nIN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE.",
        row.get::<_, &str>(2)
    );
    Ok(())
}

use futures::future::try_join_all;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

const TEST_URL: &str = env!("TEST_URL");

#[async_std::test]
async fn async_runtime() -> Result<(), Box<dyn Error>> {
    use async_std::task::spawn;
    let (client, conn) = async_postgres::connect(&TEST_URL.parse()?).await?;
    spawn(conn);
    let shared_client = Arc::new(client);
    let stmt = shared_client
        .prepare("SELECT * FROM posts WHERE id=$1")
        .await?;
    let start = Instant::now();
    let tasks = (0..1000).map(|_| {
        let client = shared_client.clone();
        let stmt = stmt.clone();
        spawn(async move {
            let queries = (0..100).map(|_| client.query_one(&stmt, &[&1i32]));
            try_join_all(queries).await
        })
    });
    let results = try_join_all(tasks).await?;
    let avg_elapsed = start.elapsed().as_micros() / 100_000;
    // check
    for rows in results {
        for row in rows {
            assert_eq!("MIT LICENSE", row.get::<_, &str>(1));
            assert_eq!(
                "Permission is hereby granted, free of charge, to any\nperson obtaining a copy of this software and associated\ndocumentation files (the \"Software\"), to deal in the\nSoftware without restriction, including without\nlimitation the rights to use, copy, modify, merge,\npublish, distribute, sublicense, and/or sell copies of\nthe Software, and to permit persons to whom the Software\nis furnished to do so, subject to the following\nconditions:\n\nThe above copyright notice and this permission notice\nshall be included in all copies or substantial portions\nof the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF\nANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED\nTO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A\nPARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT\nSHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\nCLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION\nOF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR\nIN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE.",
                row.get::<_, &str>(2)
            );
        }
    }
    println!("1000 concurrency; {} us/q", avg_elapsed);
    Ok(())
}

#[tokio::test]
async fn tokio_runtime() -> Result<(), Box<dyn Error>> {
    use tokio::spawn;
    let (client, conn) = async_postgres::connect(&TEST_URL.parse()?).await?;
    spawn(conn);
    let shared_client = Arc::new(client);
    let stmt = shared_client
        .prepare("SELECT * FROM posts WHERE id=$1")
        .await?;
    let start = Instant::now();
    let tasks = (0..1000).map(|_| {
        let client = shared_client.clone();
        let stmt = stmt.clone();
        spawn(async move {
            let queries = (0..100).map(|_| client.query_one(&stmt, &[&1i32]));
            try_join_all(queries).await
        })
    });
    let results = try_join_all(tasks).await?;
    let avg_elapsed = start.elapsed().as_micros() / 100_000;
    // check
    for rows in results {
        for row in rows? {
            assert_eq!("MIT LICENSE", row.get::<_, &str>(1));
            assert_eq!(
                "Permission is hereby granted, free of charge, to any\nperson obtaining a copy of this software and associated\ndocumentation files (the \"Software\"), to deal in the\nSoftware without restriction, including without\nlimitation the rights to use, copy, modify, merge,\npublish, distribute, sublicense, and/or sell copies of\nthe Software, and to permit persons to whom the Software\nis furnished to do so, subject to the following\nconditions:\n\nThe above copyright notice and this permission notice\nshall be included in all copies or substantial portions\nof the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF\nANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED\nTO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A\nPARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT\nSHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\nCLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION\nOF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR\nIN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE.",
                row.get::<_, &str>(2)
            );
        }
    }
    println!("1000 concurrency; {} us/q", avg_elapsed);
    Ok(())
}

#[tokio::test]
async fn tokio_postgres() -> Result<(), Box<dyn Error>> {
    use tokio::spawn;
    use tokio_postgres::NoTls;
    let (client, conn) = tokio_postgres::connect(&TEST_URL, NoTls).await?;
    spawn(conn);
    let shared_client = Arc::new(client);
    let stmt = shared_client
        .prepare("SELECT * FROM posts WHERE id=$1")
        .await?;
    let start = Instant::now();
    let tasks = (0..1000)
        .map(|_| {
            let client = shared_client.clone();
            let stmt = stmt.clone();
            spawn(async move {
                let queries = (0..100).map(|_| client.query_one(&stmt, &[&1i32]));
                try_join_all(queries).await
            })
        })
        .collect::<Vec<_>>();
    let results = try_join_all(tasks).await?;
    let avg_elapsed = start.elapsed().as_micros() / 100_000;
    // check
    for rows in results {
        for row in rows? {
            assert_eq!("MIT LICENSE", row.get::<_, &str>(1));
            assert_eq!(
                "Permission is hereby granted, free of charge, to any\nperson obtaining a copy of this software and associated\ndocumentation files (the \"Software\"), to deal in the\nSoftware without restriction, including without\nlimitation the rights to use, copy, modify, merge,\npublish, distribute, sublicense, and/or sell copies of\nthe Software, and to permit persons to whom the Software\nis furnished to do so, subject to the following\nconditions:\n\nThe above copyright notice and this permission notice\nshall be included in all copies or substantial portions\nof the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF\nANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED\nTO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A\nPARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT\nSHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\nCLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION\nOF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR\nIN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\nDEALINGS IN THE SOFTWARE.",
                row.get::<_, &str>(2)
            );
        }
    }
    println!("1000 concurrency; {} us/q", avg_elapsed);
    Ok(())
}

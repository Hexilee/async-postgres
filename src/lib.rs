//! A runtime-independent, asynchronous PostgreSQL client.

#![warn(missing_docs)]

pub use stream::AsyncStream;
pub use tokio_postgres::*;

use std::io;
use stream::connect_stream;
use tokio_postgres::tls::{NoTls, NoTlsStream, TlsConnect};
use tokio_postgres::{Client, Connection};

/// Connect to postgres server.
///
/// ```rust
/// use async_postgres::connect;
///
/// use std::error::Error;
/// use async_std::task::spawn;
///
/// async fn play() -> Result<(), Box<dyn Error>> {
///     let url = "host=localhost user=postgres";
///     let (client, conn) = connect(url.parse()?).await?;
///     spawn(conn);
///     let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
///     let value: &str = row.get(0);
///     println!("value: {}", value);
///     Ok(())
/// }
/// ```
#[inline]
pub async fn connect(
    config: Config,
) -> io::Result<(Client, Connection<AsyncStream, NoTlsStream>)> {
    connect_tls(config, NoTls).await
}

/// Connect to postgres server with a tls connector.
///
/// ```rust
/// use async_postgres::connect;
///
/// use std::error::Error;
/// use async_std::task::spawn;
///
/// async fn play() -> Result<(), Box<dyn Error>> {
///     let url = "host=localhost user=postgres";
///     let (client, conn) = connect(url.parse()?).await?;
///     spawn(conn);
///     let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
///     let value: &str = row.get(0);
///     println!("value: {}", value);
///     Ok(())
/// }
/// ```
#[inline]
pub async fn connect_tls<T>(
    config: Config,
    tls: T,
) -> io::Result<(Client, Connection<AsyncStream, T::Stream>)>
where
    T: TlsConnect<AsyncStream>,
{
    let stream = connect_stream(&config).await?;
    config
        .connect_raw(stream, tls)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

mod stream;

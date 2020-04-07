//! A runtime-independent, asynchronous PostgreSQL client.

#![warn(missing_docs)]

#[doc(inline)]
pub use connect::connect_tls;
#[doc(inline)]
pub use socket::Socket;
pub use tokio_postgres::*;

use std::io;
use tokio_postgres::tls::{NoTls, NoTlsStream};
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
) -> io::Result<(Client, Connection<Socket, NoTlsStream>)> {
    connect_tls(config, NoTls).await
}

mod connect;
mod socket;

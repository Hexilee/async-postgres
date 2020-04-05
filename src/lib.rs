//! A runtime-independent, asynchronous PostgreSQL client.

#![warn(missing_docs)]

#[doc(no_inline)]
pub use tokio_postgres::*;

#[doc(inline)]
pub use tls::{connect_with, Connection, TlsConfig};

use std::io;

/// Connect to postgres server with default tls config.
///
/// ```rust
/// use async_postgres::connect;
///
/// use std::error::Error;
/// use async_std::task::spawn;
///
/// async fn play() -> Result<(), Box<dyn Error>> {
///     let url = "host=localhost user=postgres";
///     let (client, conn) = connect(&url.parse()?).await?;
///     spawn(conn);
///     let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
///     let value: &str = row.get(0);
///     println!("value: {}", value);
///     Ok(())
/// }
/// ```
#[inline]
pub async fn connect(config: &Config) -> io::Result<(Client, Connection)> {
    connect_with(config, TlsConfig::default()).await
}

mod stream;
mod tls;

#[cfg(unix)]
use async_std::os::unix::net::UnixStream;

use crate::Socket;
use async_std::io;
use async_std::net::TcpStream;
use std::future::Future;
use std::time::Duration;
use tokio_postgres::config::{Config, Host};
use tokio_postgres::tls::{MakeTlsConnect, TlsConnect};
use tokio_postgres::{Client, Connection};

/// Default socket port of postgres.
const DEFAULT_PORT: u16 = 5432;

/// Connect to postgres server with a tls connector.
///
/// ```rust
/// use async_postgres::connect_tls;
/// use native_tls::{Certificate, TlsConnector};
/// use postgres_native_tls::MakeTlsConnector;
/// use std::fs;
/// use std::error::Error;
/// use async_std::task::spawn;
///
/// async fn play() -> Result<(), Box<dyn Error>> {
///     let cert = fs::read("database_cert.pem")?;
///     let cert = Certificate::from_pem(&cert)?;
///     let connector = TlsConnector::builder()
///         .add_root_certificate(cert)
///         .build()?;
///     let connector = MakeTlsConnector::new(connector);
///     let url = "host=localhost user=postgres sslmode=require";
///     let (client, conn) = connect_tls(url.parse()?, connector).await?;
///     spawn(conn);
///     let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
///     let value: &str = row.get(0);
///     println!("value: {}", value);
///     Ok(())
/// }
/// ```
pub async fn connect_tls<T>(
    config: Config,
    mut tls: T,
) -> io::Result<(Client, Connection<Socket, T::Stream>)>
where
    T: MakeTlsConnect<Socket>,
{
    let mut error = io::Error::new(io::ErrorKind::Other, "host missing");
    let mut ports = config.get_ports().iter().cloned();
    for host in config.get_hosts() {
        let port = ports.next().unwrap_or(DEFAULT_PORT);
        let hostname = match host {
            #[cfg(unix)]
            Host::Unix(path) => path.as_os_str().to_str().unwrap_or(""),
            Host::Tcp(tcp) => tcp.as_str(),
        };
        let connector = tls
            .make_tls_connect(hostname)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        match connect_once(&config, host, port, connector).await {
            Err(err) => error = err,
            ok => return ok,
        }
    }
    Err(error)
}

async fn connect_once<T>(
    config: &Config,
    host: &Host,
    port: u16,
    tls: T,
) -> io::Result<(Client, Connection<Socket, T::Stream>)>
where
    T: TlsConnect<Socket>,
{
    let dur = config.get_connect_timeout();
    let socket = connect_socket(host, port, dur).await?;
    config
        .connect_raw(socket, tls)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

async fn connect_socket(
    host: &Host,
    port: u16,
    dur: Option<&Duration>,
) -> io::Result<Socket> {
    match host {
        #[cfg(unix)]
        Host::Unix(path) => {
            let sock = path.join(format!(".s.PGSQL.{}", port));
            let fut = UnixStream::connect(sock);
            let socket = timeout(dur, fut).await?;
            Ok(socket.into())
        }
        Host::Tcp(tcp) => {
            let fut = TcpStream::connect((tcp.as_str(), port));
            let socket = timeout(dur, fut).await?;
            socket.set_nodelay(true)?;
            Ok(socket.into())
        }
    }
}

async fn timeout<F, T>(dur: Option<&Duration>, fut: F) -> io::Result<T>
where
    F: Future<Output = io::Result<T>>,
{
    if let Some(timeout) = dur {
        io::timeout(*timeout, fut).await
    } else {
        fut.await
    }
}

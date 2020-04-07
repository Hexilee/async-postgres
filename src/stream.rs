#[cfg(unix)]
use async_std::os::unix::net::UnixStream;

use async_std::io::{self, Read, Write};
use async_std::net::TcpStream;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_postgres::config::{Config, Host};

/// Default socket port of postgres.
const DEFAULT_PORT: u16 = 5432;

/// A alias for 'static + Unpin + Send + Read + Write
pub trait AsyncReadWriter: 'static + Unpin + Send + Read + Write {}

impl<T> AsyncReadWriter for T where T: 'static + Unpin + Send + Read + Write {}

/// A adaptor between futures::io::{AsyncRead, AsyncWrite} and tokio::io::{AsyncRead, AsyncWrite}.
pub struct AsyncStream(Box<dyn AsyncReadWriter>);

impl<T> From<T> for AsyncStream
where
    T: AsyncReadWriter,
{
    fn from(stream: T) -> Self {
        Self(Box::new(stream))
    }
}

impl AsyncRead for AsyncStream {
    #[inline]
    unsafe fn prepare_uninitialized_buffer(&self, _buf: &mut [MaybeUninit<u8>]) -> bool {
        false
    }

    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for AsyncStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

/// Establish connection to postgres server by AsyncStream.
///
///
#[inline]
pub async fn connect_stream(config: &Config) -> io::Result<AsyncStream> {
    let mut error = io::Error::new(io::ErrorKind::Other, "host missing");
    let mut ports = config.get_ports().iter().cloned();
    for host in config.get_hosts() {
        let result = match host {
            #[cfg(unix)]
            Host::Unix(path) => UnixStream::connect(path).await.map(Into::into),
            Host::Tcp(tcp) => {
                let port = ports.next().unwrap_or(DEFAULT_PORT);
                TcpStream::connect((tcp.as_str(), port))
                    .await
                    .map(Into::into)
            }
            #[cfg(not(unix))]
            Host::Unix(_) => {
                io::Error::new(io::ErrorKind::Other, "unix domain socket is unsupported")
            }
        };
        match result {
            Err(err) => error = err,
            stream => return stream,
        }
    }
    Err(error)
}

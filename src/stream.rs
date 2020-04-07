use async_std::io::{self, Read, Write};
use async_std::net::TcpStream;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_postgres::config::{Config, Host};

/// Default port of postgres.
const DEFAULT_PORT: u16 = 5432;

/// A wrapper for async_std::net::TcpStream, implementing tokio::io::{AsyncRead, AsyncWrite}.
pub struct AsyncStream(TcpStream);

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
#[inline]
pub async fn connect_stream(config: &Config) -> io::Result<AsyncStream> {
    let host = try_tcp_host(&config)?;
    let port = config
        .get_ports()
        .iter()
        .copied()
        .next()
        .unwrap_or(DEFAULT_PORT);

    let tcp_stream = TcpStream::connect((host, port)).await?;
    Ok(AsyncStream(tcp_stream))
}

/// Try to get TCP hostname from postgres config.
#[inline]
fn try_tcp_host(config: &Config) -> io::Result<&str> {
    match config
        .get_hosts()
        .iter()
        .filter_map(|host| {
            if let Host::Tcp(value) = host {
                Some(value)
            } else {
                None
            }
        })
        .next()
    {
        Some(host) => Ok(host),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "At least one tcp hostname is required",
        )),
    }
}

use async_std::io::{self, Read, Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A alias for 'static + Unpin + Send + Read + Write
pub trait AsyncReadWriter: 'static + Unpin + Send + Read + Write {}

impl<T> AsyncReadWriter for T where T: 'static + Unpin + Send + Read + Write {}

/// A adaptor between futures::io::{AsyncRead, AsyncWrite} and tokio::io::{AsyncRead, AsyncWrite}.
pub struct Socket(Box<dyn AsyncReadWriter>);

impl<T> From<T> for Socket
where
    T: AsyncReadWriter,
{
    fn from(stream: T) -> Self {
        Self(Box::new(stream))
    }
}

impl AsyncRead for Socket {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let rawBuf = buf.filled_mut();
        Pin::new(&mut self.0).poll_read(cx, rawBuf).map_ok(|_| ())
    }
}

impl AsyncWrite for Socket {
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

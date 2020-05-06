use crate::frame;
use crate::Response;
use bytes::BytesMut;
use futures_core::Stream;
use futures_io::{AsyncRead, AsyncWrite};
use futures_task::{Context, Poll};
use pin_project_lite::pin_project;
use std::io;
use std::pin::Pin;

pin_project! {
    pub struct RespStream<S> {
        #[pin]
        stream: S,
        lenght: i64,
        items: i64,
        r_buffer: BytesMut,
    }
}

impl<S> RespStream<S> {
    pub fn with_stream(stream: S) -> Self {
        Self {
            stream,
            lenght: 0,
            items: 0,
            r_buffer: BytesMut::new(),
        }
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for RespStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        let stream: Pin<&mut S> = this.stream;
        stream.poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for RespStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        let stream: Pin<&mut S> = this.stream;
        stream.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        let stream: Pin<&mut S> = this.stream;
        stream.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        let stream: Pin<&mut S> = this.stream;
        stream.poll_close(cx)
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> Stream for RespStream<S> {
    type Item = io::Result<Option<Response>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let stream: Pin<&mut S> = this.stream;
        let buffer: &mut BytesMut = this.r_buffer;
        let mut buf = [0u8; 4096];
        match stream.poll_read(cx, &mut buf) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(b)) if b == 0 => {
                let err = io::Error::new(io::ErrorKind::InvalidData, "received 0 bytes");
                return Poll::Ready(Some(Err(err)));
            }
            Poll::Ready(Ok(b)) => {
                buffer.extend(&buf[..b]);
            }
            Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
        }
        loop {
            if let Some(index) = buffer.iter().position(|b| b == &b'\r') {
                let line = buffer.split_to(index + 1);
            }
            break Poll::Ready(None)
        }
    }
}

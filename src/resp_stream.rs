use crate::frame::{ERROR, SIMPLE_STRING};
use bytes::BytesMut;
use futures_core::Stream;
use futures_io::{AsyncRead, AsyncWrite};
use futures_task::{Context, Poll};
use pin_project_lite::pin_project;
use std::io;
use std::pin::Pin;

const BUFFER_SIZE: usize = 1024 * 8;

pub type Response = BytesMut;

pin_project! {
    pub struct RespStream<S> {
        #[pin]
        stream: S,
        r_buffer: BytesMut,
    }
}

impl<S> RespStream<S> {
    pub fn with_stream(stream: S) -> Self {
        Self {
            stream,
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
        // TODO: Decode buffer read  <03-05-20, alex179ohm> //
        todo!()
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for RespStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = unsafe { self.get_unchecked_mut() };
        Pin::new(&mut this.stream).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = unsafe { self.get_unchecked_mut() };
        Pin::new(&mut this.stream).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = unsafe { self.get_unchecked_mut() };
        Pin::new(&mut this.stream).poll_close(cx)
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> Stream for RespStream<S> {
    type Item = io::Result<Option<Response>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO: Add Stream support  <03-05-20, alex179ohm> //
        todo!();
    }
}

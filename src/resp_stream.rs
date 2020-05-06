use crate::frame;
use crate::Response;
use bytes::BytesMut;
use futures_core::Stream;
use futures_io::{AsyncRead, AsyncWrite};
use futures_task::{Context, Poll};
use std::io;
use std::pin::Pin;

pub struct RespStream<S> {
    stream: S,
    lenght: i64,
    items: i64,
    r_buffer: BytesMut,
}

impl<S> RespStream<S> {
    pin_utils::unsafe_pinned!(stream: S);
    pin_utils::unsafe_unpinned!(r_buffer: BytesMut);
    pin_utils::unsafe_unpinned!(lenght: i64);
    pin_utils::unsafe_unpinned!(items: i64);

    fn project<'a>(
        self: Pin<&'a mut Self>,
    ) -> (Pin<&'a mut S>, &'a mut BytesMut, &mut i64, &mut i64) {
        unsafe {
            let this = self.get_unchecked_mut();
            (
                Pin::new_unchecked(&mut this.stream),
                &mut this.r_buffer,
                &mut this.lenght,
                &mut this.items,
            )
        }
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
        let (stream, ..) = self.project();
        stream.poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for RespStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let (stream, ..) = self.project();
        stream.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let (stream, ..) = self.project();
        stream.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let (stream, ..) = self.project();
        stream.poll_close(cx)
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> Stream for RespStream<S> {
    type Item = io::Result<Option<Response>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = Pin::into_inner(self);
        let mut did_read = false;
        let mut buf = [0u8; 4096];
        loop {
            if let Some(index) = this.r_buffer.iter().position(|b| b == &b'\r') {
                let ln = this.r_buffer.len();
                let line = this.r_buffer.split_to(index);
                let _n = match line[0] {
                    frame::ERROR => {
                        let err;
                        if let Ok(err_str) = std::str::from_utf8(&this.r_buffer[1..ln - 2]) {
                            err = io::Error::new(io::ErrorKind::Other, err_str);
                        } else {
                            err = io::Error::new(
                                io::ErrorKind::InvalidData,
                                "encoding redis error string",
                            )
                        }
                        return Poll::Ready(Some(Err(err)));
                    }
                    frame::ARRAY => {}
                    _ => {}
                };
            }
            let stream_pin: Pin<&mut S> = Pin::new(&mut this.stream);
            match stream_pin.poll_read(cx, &mut buf) {
                Poll::Pending => break Poll::Pending,
                Poll::Ready(Ok(b)) if b == 0 => {
                    if did_read {
                        return Poll::Pending;
                    } else {
                        let err = io::Error::new(io::ErrorKind::InvalidData, "received 0 bytes");
                        return Poll::Ready(Some(Err(err)));
                    }
                }
                Poll::Ready(Ok(b)) => {
                    this.r_buffer.extend(&buf[..b]);
                    did_read = true;
                    continue;
                }
                Poll::Ready(Err(e)) => break Poll::Ready(Some(Err(e))),
            }
        }
    }
}

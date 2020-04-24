use crate::codec::Encoder;
use crate::frame::{BUFFER_SIZE, ERROR, SIMPLE_STRING};
use bytes::BytesMut;
use futures::io::{AsyncRead, AsyncWrite};
use std::io;
use std::str;

pub struct RespStream<S> {
    stream: S,
    recv_buf: BytesMut,
}

impl<S> RespStream<S> {
    fn with_stream(stream: S) -> Self {
        RESPStream {
            stream,
            recv_buf: BytesMut::new(),
        }
    }
}
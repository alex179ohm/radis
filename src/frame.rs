use bytes::{BytesMut, BufMut};
use std::convert::TryFrom;
use futures::io::AsyncWrite;
use futures::io::AsyncWriteExt;
use std::io;

pub(crate) const SIMPLE_STRING: u8 = b'+';
pub(crate) const ERROR: u8 = b'-';
pub(crate) const INTEGER: u8 = b':';
pub(crate) const BULK_STRING: u8 = b'$';
pub(crate) const ARRAY: u8 = b'*';
pub(crate) const CRLN: &[u8; 2] = b"\r\n";
//pub(crate) const BUFFER_SIZE: usize = 1024 * 8;

#[derive(Debug)]
pub struct Frame {
    inner: BytesMut,
}

/// From str|String to Frame.
/// 
/// Allocates a Vec of &str, and copies it in inner BytesMut buffer encoded as Array of Bulk Strings.
/// # Examples
/// ```rust,no-run
/// let frame = "AUTH passwd".into();
/// ```
impl<F: AsRef<str>> From<F> for Frame {
    fn from(s: F) -> Self {
        let s = s.as_ref().split(' ').collect::<Vec<&str>>();

        let mut len = s.len();
        let mut buf = BytesMut::new();

        buf.put(&[ARRAY][..]);

        while len > 0 {
            let n = u8::try_from((len % 10) + 48).expect("falied to convert usize to u8");
            buf.put(&[n][..]);
            len /= 10;
        }
        buf.put(&CRLN[..]);

        s.into_iter().for_each(|s| {
            let mut len = s.len();
            buf.put(&[BULK_STRING][..]);

            while len > 0 {
                let n = u8::try_from((len % 10) + 48).expect("falied to convert usize to u8");
                buf.put(&[n][..]);
                len /= 10;
            }
            buf.put(&CRLN[..]);
            buf.put(s.as_bytes());
            buf.put(&CRLN[..]);
        });

        Self { inner: buf }
    }
}

impl AsRef<[u8]> for Frame {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl Frame {
    pub async fn write<S: AsyncWrite + Unpin>(self, s: &mut S) -> io::Result<()> {
        s.write_all(self.as_ref()).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_frame_into() {
        let pass = "pass";
       let frame: Frame = format!("AUTH {}", pass).into();
       assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], frame.as_ref());
    }
}
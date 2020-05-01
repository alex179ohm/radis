use crate::frame::{ARRAY, BULK_STRING, CRLN};
use bytes::{BufMut, BytesMut};
use std::convert::TryFrom;

/// Cmd is a Wrapper struct around BytesMut and represents the encoded command expected by redis.
///
/// Allocates a Vec of &str, and copies it in inner BytesMut buffer encoded as Array of Bulk Strings as expected by the redis resp protocol.
/// # Errors
/// Panics if fails to convert usize to u8 on encoding bulk string lenghts.
/// # Examples
/// ```rust,no-run
/// let cmd: Cmd = "AUTH pass".into();
/// assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
/// ```
/// Or with format macro.
/// # Examples
/// ```rust,no-run
/// let pass = "mysecretpass";
/// let cmd: Cmd = format!("AUTH {}", pass).into();
/// assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
/// ```
#[derive(Debug)]
pub struct CmdBuffer(BytesMut);

#[derive(Debug)]
pub struct Cmd<S>(Vec<S>);

impl<S: AsRef<str>> Cmd<S> {
    pub fn new(s: S) -> Cmd<S> {
        Cmd(vec![s])
    }

    pub fn arg(mut self, s: S) -> Cmd<S> {
        self.0.push(s);
        self
    }

    pub fn build(self) -> CmdBuffer {
        let mut len = self.0.len();
        let mut buf = BytesMut::new();

        buf.put(&[ARRAY][..]);

        while len > 0 {
            let n = u8::try_from((len % 10) + 48).expect("falied to convert usize to u8");
            buf.put(&[n][..]);
            len /= 10;
        }
        buf.put(&CRLN[..]);

        self.0.into_iter().for_each(|s| {
            let mut len = s.as_ref().len();
            buf.put(&[BULK_STRING][..]);

            while len > 0 {
                let n = u8::try_from((len % 10) + 48).expect("falied to convert usize to u8");
                buf.put(&[n][..]);
                len /= 10;
            }
            buf.put(&CRLN[..]);
            buf.put(s.as_ref().as_bytes());
            buf.put(&CRLN[..]);
        });

        CmdBuffer(buf)
    }
}

impl AsRef<[u8]> for CmdBuffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cmd_into() {
        let pass = "pass";
        let cmd = Cmd::new("AUTH").arg(pass).build();
        assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
    }
}

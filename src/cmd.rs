use crate::frame::{ARRAY, BULK_STRING, CRLN};
use bytes::{BufMut, BytesMut};
use std::convert::TryFrom;
use std::io;
use std::num::TryFromIntError;

struct CmdEncodingError(TryFromIntError);

impl From<TryFromIntError> for CmdEncodingError {
    fn from(e: TryFromIntError) -> Self {
        CmdEncodingError(e)
    }
}

impl From<CmdEncodingError> for io::Error {
    fn from(e: CmdEncodingError) -> Self {
        io::Error::new(io::ErrorKind::Other, e.0)
    }
}

/// Represents Redis Command and it Arguments.
///
/// # Examples
/// ```rust
/// # use radis::Cmd;
/// let pass = "pass";
/// let cmd = Cmd::new("AUTH").arg(pass).build().expect("failed to create auth command");
/// assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
/// ```
pub struct Cmd<S>(Vec<S>);

impl<S: AsRef<str>> Cmd<S> {
    /// Creates a new Cmd struct with the cmd :
    pub fn new(cmd: S) -> Cmd<S> {
        Cmd(vec![cmd])
    }

    pub fn arg(mut self, s: S) -> Cmd<S> {
        self.0.push(s);
        self
    }

    pub fn build(self) -> Result<CmdBuffer, io::Error> {
        let mut len = self.0.len();
        let mut buf = BytesMut::new();

        buf.put(&[ARRAY][..]);

        while len > 0 {
            let n = u8::try_from((len % 10) + 48)
                .map_err(|e| io::Error::from(CmdEncodingError::from(e)))?;
            buf.put(&[n][..]);
            len /= 10;
        }
        buf.put(&CRLN[..]);

        for s in self.0 {
            let mut len = s.as_ref().len();
            buf.put(&[BULK_STRING][..]);

            while len > 0 {
                let n = u8::try_from((len % 10) + 48)
                    .map_err(|e| io::Error::from(CmdEncodingError::from(e)))?;
                buf.put(&[n][..]);
                len /= 10;
            }
            buf.put(&CRLN[..]);
            buf.put(s.as_ref().as_bytes());
            buf.put(&CRLN[..]);
        }

        Ok(CmdBuffer(buf))
    }
}

/// The Encoded Redis command buffer.
#[derive(Debug)]
pub struct CmdBuffer(BytesMut);

impl AsRef<[u8]> for CmdBuffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

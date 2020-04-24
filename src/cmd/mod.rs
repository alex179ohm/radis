mod connection;

#[cfg(feature = "geo")]
mod geo;

pub use connection::*;

#[cfg(feature = "geo")]
pub use geo::*;

pub trait ToCmd {
    fn to_cmd(&self) -> Cmd;
}

pub struct Cmd {
    inner: Vec<BufferedFrame>,
}

impl Cmd {
    fn cmd(c: &str) -> Cmd {
        let mut v = Vec::new();
        v.push(BufferedFrame::Bulk(c.to_owned()));
        Cmd { inner: v }
    }
    fn arg<ARG: Into<String>>(self, a: ARG) -> Cmd {
        self.inner.push(BufferedFrame::Bulk(a.into()));
        self
    }
}

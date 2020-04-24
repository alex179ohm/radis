use super::{Cmd, ToCmd};

pub struct Auth<'a>(&'a str);

impl<'a> Auth<'a> {
    pub fn with(passwd: &'a str) -> Self {
        Self(passwd)
    }
}

impl<'a> ToCmd for Auth<'a> {
    fn to_cmd(&self) -> Cmd {
        Cmd::cmd("AUTH").arg(self.0)
    }
}

pub struct Echo<'a>(&'a str);

impl<'a> Echo<'a> {
    pub fn with(msg: &'a str) -> Self {
        Self(msg)
    }
}

impl<'a> ToCmd for Echo<'a> {
    fn to_cmd(&self) -> Cmd {
        Cmd::cmd("ECHO").arg(self.0)
    }
}

pub struct Quit;

impl ToCmd for Quit {
    fn to_cmd(&self) -> Cmd {
        Cmd::cmd("QUIT")
    }
}

pub struct Ping<'a>(Option<&'a str>);

impl<'a> Ping<'a> {
    pub fn with(msg: Option<&'a str>) -> Self {
        Self(msg)
    }
}

impl<'a> ToCmd for Ping<'a> {
    fn to_cmd(&self) -> Cmd {
        let cmd = Cmd::cmd("PING");
        match self.0 {
            Some(msg) => cmd.arg(msg),
            None => cmd,
        }
    }
}

pub struct Select<'a>(&'a str);

impl<'a> Select<'a> {
    pub fn with(db: &'a str) -> Self {
        Self(db)
    }
}

impl<'a> ToCmd for Select<'a> {
    fn to_cmd(&self) -> Cmd {
        Cmd::cmd("SELECT").arg(self.0)
    }
}

pub struct SwapDB<'a>(&'a str);

impl<'a> SwapDB<'a> {
    pub fn with(db: &'a str) -> Self {
        Self(db)
    }
}

impl<'a> ToCmd for SwapDB<'a> {
    fn to_cmd(&self) -> Cmd {
        Cmd::cmd("SWAPDB").arg(self.0)
    }
}

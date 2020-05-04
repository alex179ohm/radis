use radis::Cmd;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cmd_into() {
        let pass = "pass";
        let cmd = Cmd::new("AUTH")
            .arg(pass)
            .build()
            .expect("failed to create command");
        assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
    }
}

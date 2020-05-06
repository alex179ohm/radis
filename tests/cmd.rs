#[cfg(test)]
mod test {
    use radis::Cmd;

    #[test]
    fn cmd_new_test() {
        let cmd = Cmd::new("AUTH");
        assert_ne!(cmd, Cmd::new("NOAUTH"));
    }

    #[test]
    fn cmd_arg_test() {
        let cmd = Cmd::new("AUTH").arg("pass");
        assert_ne!(cmd, Cmd::new("AUTH").arg("reset"));
    }

    #[test]
    fn cmd_build_into() {
        let pass = "pass";
        let cmd = Cmd::new("AUTH")
            .arg(pass)
            .build()
            .expect("failed to create command");
        assert_eq!(&b"*2\r\n$4\r\nAUTH\r\n$4\r\npass\r\n"[..], cmd.as_ref());
    }
}

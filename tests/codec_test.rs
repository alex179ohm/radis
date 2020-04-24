use bytes::BytesMut;
use redis_std::codec::Encoder;
use redis_std::{Auth, GeoAdd, Quit};

#[test]
fn one_argument_test() {
    let mut buf = BytesMut::new();
    Auth::with("authtestpass").encode(&mut buf);
    assert_eq!(
        &b"*2\r\n$4\r\nAUTH\r\n$12\r\nauthtestpass\r\n"[..],
        &buf[..]
    );
}

#[test]
fn no_argument_test() {
    let mut buf = BytesMut::new();
    Quit {}.encode(&mut buf);
    assert_eq!(&b"*1\r\n$4\r\nQUIT\r\n"[..], &buf[..])
}

#[test]
fn many_arguments_test() {
    let mut buf = BytesMut::new();
    GeoAdd::with(
        "Sicily",
        "13.361389",
        "38.115556",
        "Palermo",
        &["15.087269", "37.502669", "Catania"],
    )
    .encode(&mut buf);
    eprintln!("{:?}", String::from_utf8(buf.to_vec()));
    assert_eq!(&b"*8\r\n$6\r\nGEOADD\r\n$6\r\nSicily\r\n$9\r\n13.361389\r\n$9\r\n38.115556\r\n$7\r\nPalermo\r\n$9\r\n15.087269\r\n$9\r\n37.502669\r\n$7\r\nCatania\r\n"[..],
        &buf[..]);
}

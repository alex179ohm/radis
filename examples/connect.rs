use async_std::net::TcpStream;
use radis::Cmd;
use std::io;

#[async_std::main]
async fn main() -> io::Result<()> {
    let tcp = TcpStream::connect("127.0.0.1:6379").await?;
    let mut conn = Conn::with_stream(tcp);

    let cmd = Cmd::new("AUTH").arg("secretpass").build();

    println!("{:?}", cmd);
    Ok(())
}

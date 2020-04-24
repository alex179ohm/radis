use async_std::net::TcpStream;
use redis_std::Conn;
use std::io;

#[async_std::main]
async fn main() -> io::Result<()> {
    let tcp = TcpStream::connect("127.0.0.1:6379").await?; 
    let mut conn = Conn::with_stream(tcp);

    let s = conn.auth("testpass").await?;

    println!("{}", s);
    Ok(())
}

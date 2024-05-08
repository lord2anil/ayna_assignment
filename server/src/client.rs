

use core::str;

use tokio::{io::AsyncWriteExt,  net::TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use warp::filters::ws::WebSocket;
const ECHO_SERVER:&str="localhost:9001";






#[tokio::main]

async fn main() {
   println!("connecting to {}",ECHO_SERVER);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER).await {
        println!("connected to echo server {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        
        // convert stream to websocket stream
        let stream = tokio_tungstenite::accept_async(stream).await;
        
        // Rest of the code...
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER);
    }
  



}

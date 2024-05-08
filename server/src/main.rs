use std::net::TcpListener;
use std::thread::spawn;

use serde::Serialize;
use serde::Deserialize;

use tungstenite::accept;
use tungstenite::Message;
use reqwest;


// use websocket::client::r#async;
use tokio;

use reqwest::{Client, header::CONTENT_TYPE};

use serde_json::json;
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    msg: String,
}

// this function is used to save the message to the database
async fn save_msg(dt:&str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let user_msg = json!(
        {
            "data":{
                "msg": dt
            
            }
        }
    );
    let response = client
        .post("http://localhost:1337/api/messages")
        .header(CONTENT_TYPE, "application/json")
        .json(&user_msg)
        .send()
        .await?;

    if response.status().is_success() {
        println!("User created successfully!");
        let response_body: serde_json::Value = response.json().await?;
        println!("Response body: {:?}", response_body);
    } else {
        println!("Request failed: {}", response.status());
    }

    Ok(())
}

/// A WebSocket echo server
#[tokio::main]
 async fn main ()->() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Server is running");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                // receive the message from the client and send it back to the client
                let msg = websocket.read().unwrap();
                let msg1 = msg.clone();
                if msg1.to_string()!=""{
                let  rt= tokio::runtime::Runtime::new().unwrap();
                rt.block_on(save_msg(msg.to_text().unwrap())).unwrap();
                }
                websocket.send(Message::from(msg1)).unwrap();
            }
        });
    }
}
use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{stdin,Read};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    member: String,
    chat: String,
}
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("{:?}", stream);

    let mut buff = [0; 1024];
    stream.read(&mut buff)?;
    let mut buffer: Vec<u8> = Vec::new();
    for &i in buff.iter() {
        if i != 0 {
            buffer.push(i);
        }else{
            break;
        }
    }
    let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    println!("{:?}", result.member);
    if result.member == "root".to_string() {
        let message = serde_json::to_string(&Message {
            member: "test".to_string(),
            chat: "Welcome".to_string(),
        })?;
        println!("{:?}", message);
        stream.write_all(message.as_bytes())?;
        stream.flush()?;
        print!("Flushed");
    }
    // loop{
        print!("Enter Your Message :");
        let mut res = String::new();
        // std::io
        let mut std = stdin();
        std.read_line(&mut res)?;
        if res.to_lowercase()=="exit"{
            // break;
            // continue;
            // Ok(())
            // return ;
            print!("Exit");
        }else{
            let message = serde_json::to_string(&Message {
                member: "test".to_string(),
                chat: res,
            })?;
            println!("{:?}", message);
            stream.write_all(message.as_bytes())?;
            stream.flush()?;
        // }
        // break;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpStream::connect("127.0.0.1:8080")?;
    handle_client(listener)?;
    Ok(())
}

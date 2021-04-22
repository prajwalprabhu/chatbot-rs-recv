use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::{stdin, Read};
use std::net::TcpStream;
use std::thread;

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
        } else {
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
    let stream_clone = stream.try_clone()?;
    println!("Spawning threads");
    let recv_thread = thread::spawn(|| {
        recv(stream_clone).expect("recv thread said ");
        println!("Started recv");
    });
    let send_thread = thread::spawn(|| send(stream).expect("send thread said "));
    recv_thread.join().expect("recv join ");
    send_thread.join().expect("send join ");
    Ok(())
}
fn recv(mut stream: TcpStream) -> std::io::Result<()> {
    print!("Recieving");
    loop {
        let mut file = std::fs::File::create("output.txt").expect("Cant Open");
        let mut buff = [0; 1024];
        stream.read(&mut buff)?;
        if buff[0] == 0 {
            continue;
        }
        let mut buffer: Vec<u8> = Vec::new();
        for &i in buff.iter() {
            if i != 0 {
                buffer.push(i);
            } else {
                break;
            }
        }
        let result: Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
        println!("Message Recv {:?}", result);
        file.write_all(String::from_utf8_lossy(&buffer).as_bytes())?;
    }
    Ok(())
}
fn send(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        println!("Enter Your Message :");
        let mut res = String::new();
        // std::io
        let std = stdin();
        std.read_line(&mut res)?;
        print!("Youentered{:?} ", res);
        if res.to_lowercase() == "exit" {
            print!("Exit");
            return Ok(())
            // std::process::exit(0);

            // break;
            // continue;
            // Ok(())
            // return ;
        } else {
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
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    let listener = TcpStream::connect("127.0.0.1:8080")?;
    handle_client(listener)?;
    Ok(())
}

use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::{stdin, Read};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub member: String,
    pub chat: String,
}
pub fn handle_client(mut stream: & TcpStream)->(thread::JoinHandle<()>,mpsc::Receiver<Message>){
    println!("{:?}", stream);
    let mut buff = [0; 1024];
    stream.read(&mut buff).expect("cant read buff");
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
        }).expect("Cant convert serde_json");
        println!("{:?}", message);
        stream.write_all(message.as_bytes()).expect("Cant write to stream");
        stream.flush().expect("Cant flush");
        println!("Flushed");
    }
    let stream_clone = stream.try_clone().expect("Cant clone ");
    println!("Spawning threads");
    let (tx, rx) = mpsc::channel();
    let tx_ = tx.clone();
    let recv_thread = thread::spawn(|| {
        recv(stream_clone, tx_).expect("recv thread said ");
        println!("Created channel");
        println!("Started recv");
    });
    print!("Satrted thread");

    // let send_thread = thread::spawn(|| send(stream).expect("send thread said "));
    // recv_thread.join().expect("recv join ");
    // send_thread.join().expect("send join ");
    (recv_thread,rx)
}
fn recv(mut stream: TcpStream, channel: mpsc::Sender<Message>) -> std::io::Result<()> {
    print!("Recieving");
    loop {
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
        println!("Result :{:?} ",result);
        channel.send(result).expect("Cannel : ");
    }
    Ok(())
}
// fn send(mut stream: TcpStream) -> std::io::Result<()> {
//     loop {
//         println!("Enter Your Message :");
//         let mut res = String::new();
//         // std::io
//         let std = stdin();
//         std.read_line(&mut res)?;
//         print!("Youentered{:?} ", res);
//         if res.to_lowercase() == "exit" {
//             print!("Exit");
//             return Ok(())
//             // std::process::exit(0);
//
//             // break;
//             // continue;
//             // Ok(())
//             // return ;
//         } else {
//             let message = serde_json::to_string(&Message {
//                 member: "test".to_string(),
//                 chat: res,
//             })?;
//             println!("{:?}", message);
//             stream.write_all(message.as_bytes())?;
//             stream.flush()?;
//             // }
//             // break;
//         }
//     }
//     Ok(())
// }
pub fn send(mut stream: &TcpStream, message: Message){
    let message = serde_json::to_string(&message).expect("json error");
    stream.write_all(message.as_bytes()).expect("write err");
    stream.flush().expect("flush err");
    // Ok(())
}
// pub fn run(list: gtk::ListBox) -> TcpStream,std::io::Result<() {
//     handle_client(listener.try_clone()?, list)?;
//     (listener,Ok(()))
// }

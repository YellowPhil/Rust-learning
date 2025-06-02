use message::message::Message;
use std::{
    io::BufReader,
    io::Read,
    net::{TcpListener, TcpStream},
};
mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();

    for steam in listener.incoming() {
        let stream = steam.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut msg_contents = Vec::new();

    if let Err(e) = buf_reader.read_to_end(&mut msg_contents) {
        println!("Error reading msg: {e}");
        return;
    }

    let msg_string = match String::from_utf8(msg_contents) {
        Ok(string) => string,
        Err(e) => {
            println!("Error while converting message to string: {e}");
            return;
        }
    };

    let lines: Vec<String> = msg_string.split_terminator("\r\n")
        .map(|s| s.to_string())
        .collect();

    let msg = match Message::try_from(lines) {
        Ok(message) => message,
        Err(e) => {
            println!("Message format is incorrect: {e}");
            return;
        }
    };
    println!("{:?}", msg);
    let _  = msg.save().map_err(|e| println!("Error saving message: {e}"));
}

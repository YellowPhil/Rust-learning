use message::message::Message;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use crate::message::user;

mod message;

#[tokio::main]
async fn main() {
    let send_handle = tokio::spawn(async {
        let send_listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
        loop {
            let (socket, _) = send_listener.accept().await.unwrap();
            println!("Connection established!");
            tokio::spawn(async move {
                let _ = handle_connection(socket).await;
            });
        }
    });
    let receiver_handle = tokio::spawn(async {
        let receive_listener = TcpListener::bind("127.0.0.1:8086").await.unwrap();
        loop {
            let (socket, _) = receive_listener.accept().await.unwrap();
            println!("Receive Connection established!");
            tokio::spawn(async move {
                let _ = handle_receive(socket).await;
            });
        }
    });
    send_handle.await.unwrap();
    receiver_handle.await.unwrap()
}

async fn handle_connection(mut stream: TcpStream) {
    // let mut buf_reader = BufReader::new(stream.read()?);
    let mut msg_contents = Vec::new();
    if let Err(e) = stream.read_to_end(&mut msg_contents).await {
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

    let lines: Vec<String> = msg_string
        .split_terminator("\r\n")
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
    let _ = msg
        .save()
        .map_err(|e| println!("Error saving message: {e}"));
}

async fn handle_receive(mut stream: TcpStream) {
    let mut username_buf = Vec::new();
    stream.read_to_end(&mut username_buf).await.unwrap();
    let user = user::User::from(String::from_utf8(username_buf).unwrap());
    println!("User create: {:?}", user);
    let inbox = user.get_inbox();
    for msg in inbox {
        println!("{:?}", msg);
    }
}

use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;

struct ChatClient {
    stream: TcpStream,
}

fn get_client(stream: TcpStream) -> ChatClient {
    ChatClient { stream: stream }
}

fn main() -> std::io::Result<()> {
    let (sender, receiver) = channel::<ChatClient>();

    spawn(move || {
        let mut clients = vec![];
        let mut messages = vec![];
        loop {
            match receiver.recv_timeout(Duration::from_millis(10)) {
                Ok(client) => {
                    clients.push(client);

                    println!("Added client");
                }
                Err(_) => {}
            }
            let mut buf = [0u8; 320];
            for client in &mut clients {
                client
                    .stream
                    .set_read_timeout(Some(Duration::from_nanos(10)));
                match client.stream.read(&mut buf) {
                    Ok(len) => {
                        if len > 0 {
                            messages.push(buf);
                            println!("{:?}", buf.to_vec());
                        }
                    }
                    Err(_) => {}
                }
            }

            match messages.pop() {
                Some(message) => {
                    for client in &mut clients {
                        client.stream.write(&message);
                        client.stream.flush();
                    }
                }
                None => {}
            }
        }
    });

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let client = get_client(stream?);
        sender.send(client);
    }
    Ok(())
}

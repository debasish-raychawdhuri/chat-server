use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::env;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <2 {
        println!("Please specify your name");
        return Ok(());
    }
    let name = &args[1];
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut stream2 = stream.try_clone()?;
    spawn(move || {
        let mut buf = [0u8; 320];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    if len > 0 {
                        match String::from_utf8(buf.to_vec()) {
                            Ok(message) => {
                                println!("{}", message);
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });

    loop {
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(len) => {
                if len > 0 {
                    let bbuf = buf.as_bytes();
                    stream2.write_all(name.as_bytes());
                    stream2.write_all(": ".as_bytes());
                    stream2.write_all(&bbuf);
                }
            }
            Err(_) => {}
        }
    }
    Ok(())
}

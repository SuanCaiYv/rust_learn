extern crate core;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use text_io::scan;


fn main() {
    let handle1 = thread::spawn(|| {
        let listener_result = TcpListener::bind("127.0.0.1:8190");
        if let Ok(listener) = listener_result {
            println!("listening on port 8190");
            for incoming in listener.incoming() {
                if let Ok(socket) = incoming {
                    handle(socket);
                } else {
                    println!("error occur");
                }
            }
        } else {
            panic!("bind error");
        }
    });
    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        match TcpStream::connect("127.0.0.1:8190") {
            Ok(mut socket) => {
                let mut buffer = [0 as u8;1024];
                println!("输入一些话: ");
                for i in 0..10 {
                    let input: String;
                    scan!("{}", input);
                    let size = socket.write(input.as_bytes()).unwrap();
                    socket.flush();
                    match socket.read(&mut buffer) {
                        Ok(size) => {
                            let str = String::from_utf8_lossy(&buffer[0..size]);
                            println!("读到响应: {}", str);
                        }
                        Err(_) => {
                            socket.shutdown(Shutdown::Both).unwrap();
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                panic!("failed to connect server.")
            }
        }
    });
    handle1.join();
    handle2.join();
}

fn handle(mut socket: TcpStream) {
    let mut buffer = [0 as u8;1024];
    loop {
        match socket.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    continue;
                }
                let str = String::from_utf8_lossy(&buffer[0..size]);
                println!("服务端读到了: {}", &str);
                let resp = format!("Hello, client: {}", str);
                if let Err(_) = socket.write(resp.as_bytes()) {
                    break;
                }
            }
            Err(_) => {
                socket.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

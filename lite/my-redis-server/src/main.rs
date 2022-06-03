use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use mini_redis::Command::{Get, Set};
use tokio::net::{TcpListener, TcpStream};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:16379";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("server listen on: {}", addr);
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 实现多个连接之间共享数据
        let db0 = db.clone();
        tokio::spawn(async move {
            process(socket, db0).await;
        });
    }
}

async fn process(socket: TcpStream, db: DB) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut database = db.lock().unwrap();
                database.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let database = db.lock().unwrap();
                if let Some(val) = database.get(cmd.key()) {
                    Frame::Bulk(val.clone())
                } else {
                    Frame::Null
                }
            }
            cmd=> {
                panic!("todo implement")
            }
        };
        connection.write_frame(&response).await.unwrap();
    }
}
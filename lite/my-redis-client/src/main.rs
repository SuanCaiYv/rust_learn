use bytes::Bytes;
use mini_redis::client;
use mini_redis::client::Client;
use mini_redis::Result;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use crate::Cmd::{Get, Set};

type Response<T> = oneshot::Sender<Result<T>>;

enum Cmd {
    Get {
        key: String,
        // 通过一对一的管道获取响应结果
        response: Response<Bytes>,
    },
    Set {
        key: String,
        val: Bytes,
        response: Response<()>,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (sender1, receiver1) = mpsc::channel(64);
    let (sender2, receiver2) = mpsc::channel(64);
    let mut client1 = client::connect("127.0.0.1:16379").await.unwrap();
    let mut client2 = client::connect("127.0.0.1:16379").await.unwrap();
    let send_manager1 = tokio::spawn(async move {
        send_process(receiver1, client1).await
    });
    let send_manager2 = tokio::spawn(async move {
        send_process(receiver2, client2).await
    });
    let sender11 = sender1.clone();
    let operation1 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let cmd = Get {
            key: "key1".to_string(),
            response: resp_sender,
        };
        sender11.send(cmd).await;
        // 包装的有点多，所以，解包装也有点多
        let ans = resp_receiver.await.unwrap().unwrap();
        println!("{:?}", ans)
    });
    let sender12 = sender1.clone();
    let operation2 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let cmd = Set {
            key: "key1".to_string(),
            val: Bytes::from("aaa".to_string()),
            response: resp_sender,
        };
        sender12.send(cmd).await;
        let ans = resp_receiver.await.unwrap().unwrap();
        println!("{:?}", ans)
    });
    let sender21 = sender2.clone();
    let operation3 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let cmd = Get {
            key: "key2".to_string(),
            response: resp_sender,
        };
        sender21.send(cmd).await;
        let ans = resp_receiver.await.unwrap().unwrap();
        println!("{:?}", ans)
    });
    let sender22 = sender2.clone();
    let operation4 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();
        let cmd = Set {
            key: "key2".to_string(),
            val: Bytes::from("bbb".to_string()),
            response: resp_sender,
        };
        sender22.send(cmd).await;
        let ans = resp_receiver.await.unwrap().unwrap();
        println!("{:?}", ans)
    });
    send_manager1.await.unwrap();
    send_manager2.await.unwrap();
    // 下面的操作是异步的，加上直接解包装，可能造成Get操作对None解包装，多运行几次就行
    operation2.await.unwrap();
    operation4.await.unwrap();
    operation3.await.unwrap();
    operation1.await.unwrap();
    Ok(())
}

async fn send_process(mut receiver: mpsc::Receiver<Cmd>, mut client: Client) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            Get {key, response} => {
                if let Some(resp) = client.get(key.as_str()).await.unwrap() {
                    response.send(Ok(resp));
                }
            }
            Set {key, val, response} => {
                client.set(key.as_str(), val.clone()).await.unwrap();
                response.send(Ok(()));
            }
        }
    }
}

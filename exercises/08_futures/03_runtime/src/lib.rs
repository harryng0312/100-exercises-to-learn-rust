// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
// `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let reply_v = Arc::new(RwLock::new(reply));
    loop {
        let (mut socket1, add1) = first.accept().await.expect("accept failed");
        let (mut socket2, add2) = second.accept().await.expect("accept failed");
        // let reply_str1 = format!("{}", reply);
        // let reply_str2 = format!("{}", reply);
        let reply1 = reply_v.clone();
        let reply2 = reply_v.clone();
        tokio::spawn(async move {
            let (mut reader1, mut writer1) = socket1.split();
            writer1.write(format!("{}", reply1.read().await).as_bytes()).await.expect("write failed");
            // writer1.write(format!("{}", &reply1).as_bytes()).await.expect("write failed");
        });
        tokio::spawn(async move {
            let (mut reader2, mut writer2) = socket2.split();
            writer2.write(format!("{}", reply2.read().await).as_bytes()).await.expect("write failed");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}

use futures::SinkExt;
use tmq::{push, Context};

#[tokio::main]
async fn main() {
    let mut socket = push(&Context::new()).connect("tcp://0.0.0.0:5555").unwrap();

    let res = socket.send(vec!["hello"]).await;
    println!("{:?}", res);
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn integration_test_log_event() {
        let mut ps = push(&Context::new()).connect("tcp://0.0.0.0:5555").unwrap();

        let mut pl = tmq::pull(&Context::new())
            .bind("tcp://0.0.0.0:5555")
            .unwrap();

        let _ = ps.send(vec!["hella fly"]).await;
        match pl.next().await {
            Some(msg) => {
                let value = msg.unwrap();
                let stringy = value
                    .iter()
                    .map(|item| item.as_str().unwrap_or("invalid text"))
                    .collect::<Vec<&str>>();
                assert_eq!(stringy, vec!["hella fly"]);
            }
            None => panic!("ahhhh"),
        }
    }
}

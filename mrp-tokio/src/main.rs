use tokio_stream::StreamExt;
use tokio_tungstenite::connect_async;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let (mut socket, _) = connect_async("wss://gateway.discord.gg")
        .await
        .expect("Can't connect");

    let hello = socket.next().await.unwrap().unwrap();
    println!("{hello}");
    socket.close(None).await.unwrap();
    loop {
        match socket.next().await {
            Some(Ok(msg)) => println!("{msg}"),
            Some(Err(e)) => panic!("{e}"),
            None => break,
        };
    }
}
use tracing_subscriber::filter::LevelFilter;
use tungstenite::{connect, Error};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let (mut socket, _) = connect("wss://gateway.discord.gg").expect("Can't connect");

    let hello = socket.read_message().unwrap();
    println!("{hello}");
    socket.close(None).unwrap();
    loop {
        match socket.read_message() {
            Ok(msg) => println!("{msg}"),
            Err(Error::ConnectionClosed) => break,
            Err(e) => panic!("{e}"),
        };
    }
}

use async_std::{prelude::*, net::{TcpListener, ToSocketAddrs}, task};
use zuppa::netutils::Result;

/// Accept new connections and register them into the game (or reject them if necessary).
async fn acceptance_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream?;

        println!("Connection: {:?}", stream);

        stream.write_all(b"hello");
    }

    Ok(())
}

fn main() -> Result<()> {
    // // Initialize game.
    // use zuppa::world::builder::Builder;
    // let mut world = Builder::new()
    //     // TODO: add this to check for actors in config files with replicated usernames.
    //     // .using_server_warnings()
    //     .with_players_file("assets/players.ron")
    //     .with_phrases_dir("assets/phrases")
    //     .map(Builder::build)
    //     .expect("Error while building world.");

    task::block_on(acceptance_loop("127.0.0.1:8080"))
}

use log::error;
use std::net::TcpListener;
use text_io::read;
use zuppa::connection::*;

fn main() {
    // Initialize game.
    use zuppa::world::builder::Builder;
    let mut world = Builder::new()
        // TODO: add this to check for actors in config files with replicated usernames.
        // .using_server_warnings()
        .with_players_file("assets/players.ron")
        .with_phrases_dir("assets/phrases")
        .map(Builder::build)
        .expect("Error while building world.");

    // All player connections.
    let connections = vec![];

    let listener = TcpListener::bind("127.0.0:8787").unwrap();

    // Thread handling client registrations.
    let handle = std::thread::spawn(|| {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let new_connection = register_connection(stream, world);
            connections.push(new_connection);
        }
    });

    // Server command prompt.
    loop {
        println!("> ");
        let command = read!("{}\n");

        // TODO: make command parsing more sophisticated...
        if command == "exit" || command == "quit" {
            error!("Shutting down server...");
            return;
        } else if command == "start" {
            break;
        } else {
            println!("Invalid command: {}", command);
        }
    }

    // Start game.
    zuppa::phases::intro(world, connections.into_iter().map(|c| (cconnection, )));
}

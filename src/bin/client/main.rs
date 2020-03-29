use async_std::{io::{BufReader, BufWriter}, net::TcpStream, prelude::*, task};
use terview::{msg, msgln, View};
use text_io::read;
use zuppa::{
    netmsg,
    netutils::Result,
};
use futures::sink::SinkExt;

type NetMsg = netmsg::NetMsg<String>;

async fn main_loop(v: &mut impl View) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    handle_registration(v, &mut stream).await?;

    Ok(())
}

async fn handle_registration(v: &mut impl View, stream: &mut TcpStream) -> Result<()> {
    let mut lines = BufReader::new(stream.clone()).lines();
    let mut writer = BufWriter::new(stream.clone());

    loop {
        msg!(v, "Input character name: ");
        let actor_name: String = read!("{}\n");
        writer.write_all(actor_name.as_bytes()).await?;

        let response: NetMsg = ron::de::from_str(&lines.next().await.unwrap()?)?;
        match response {
            NetMsg::Err(err) => {
                msgln!(v, "Error: {:?}", err);
                msgln!(v, "Retrying connection...");
            },
            NetMsg::PosAwk => {
                msgln!(v, "Connected!");
                break;
            },
            _ => panic!(),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut view = terview::simple_term::SimpleTermView;

    task::block_on(main_loop(&mut view))
}

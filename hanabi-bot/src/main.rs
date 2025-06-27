mod test;

use futures_util::{SinkExt, TryStreamExt};
use hanablive::messages::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut client = hanablive::client::Client::new();

    let username = std::env::var("HANABI_USERNAME")?;
    let password = std::env::var("HANABI_PASSWORD")?;

    client.login(&username, &password).await?;
    client.connect_ws().await?;

    while let Ok(message) = client.try_next().await {
        match message {
            Some(Message::Chat(
                   hanablive::messages::ChatData {
                       msg,
                       who: Some(who),
                       room: Some(room),
                       recipient: Some(recipient),
                       ..
                   })) =>
                {
                    if !recipient.eq(&username) {
                        continue;
                    }

                    println!("PM from {}: {}", &who, &msg);
                    client.send(Message::ChatPM(hanablive::messages::ChatPMData {
                        msg: "Hello! I am a new work-in-progress H-Group bot. I don't work yet, but when I do you can DM me again and I'll explain how I work.".to_string(),
                        recipient: who,
                        room,
                    })).await?;
                }
            Some(ref _msg) => eprintln!("Unhandled message: {:?}", message),
            _ => (),
        }
    }

    Ok(())
}

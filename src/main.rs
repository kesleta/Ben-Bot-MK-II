use serenity::{async_trait, model::prelude::*, prelude::*, Client};
use std::env;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        if !msg.author.bot {
            if msg.content == "ping" {
                msg.channel_id
                    .say(&context.http, "pong")
                    .await
                    .expect("Failed to send message");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Failed to read Token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Couldn't create the new client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

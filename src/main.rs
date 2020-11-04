use std::fs;

extern crate serde;
extern crate toml;

use serde::Deserialize;

use serenity::{
    async_trait,
    model::{id::ChannelId, channel::Message, gateway::Ready},
    prelude::*,
};

#[derive(Deserialize)]
struct Config {
    token: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        //let channel_id = Channel::from_str("bot-test").unwrap();
        let channel_id = ChannelId(773179565458980865);
        let _ = channel_id.say(&ctx.http, "ready").await;
    }
}

#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    let mut client = Client::builder(&config.token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

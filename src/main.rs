#![warn(clippy::pedantic)]

use std::fs;

extern crate serde;
extern crate toml;
extern crate wandbox;

use serde::Deserialize;

use serenity::{
    async_trait,
    model::{id::ChannelId, channel::Message, gateway::Ready},
    prelude::*,
};

use wandbox::{Wandbox, CompilationBuilder};

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
        } else if msg.content.starts_with("!wandbox") {
            let src = &msg.content[9..];

            println!("src: {}", src);

            let wbox = match Wandbox::new(None, None).await {
                Ok(wbox) => wbox,
                Err(e) => return println!("{}", e),
            };
            let mut builder = CompilationBuilder::new();
            builder.target("clang-head");
            builder.options_str(vec!["-Wall", "-Werror"]);
            builder.code(src);
            let _result = match builder.build(&wbox) {
                Ok(res) => res,
                Err(e) => {
                    let _ = msg.channel_id.say(&ctx.http, &e);
                    return println!("{}", e);
                },
            };
            let result = builder.dispatch().await.expect("Failed to lookup");
            println!("compiler: {}", result.compiler_all);
            println!("program: {}", result.program_all);

            let _ = msg.channel_id.say(&ctx.http, result.compiler_all).await;
            let _ = msg.channel_id.say(&ctx.http, result.program_all.replace("@", "＠").replace("!wandbox", "！wandbox")).await;
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
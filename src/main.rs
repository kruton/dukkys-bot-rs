mod commands;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

use commands::help::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(help)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected TOKEN environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating Discord client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

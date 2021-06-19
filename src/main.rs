extern crate dotenv;

use std::env;
use dotenv::dotenv;

use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN")
        .expect("Expected TOKEN environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating Discord client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

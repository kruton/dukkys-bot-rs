extern crate dotenv;

use std::env;
use dotenv::dotenv;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN")
        .expect("Expected TOKEN environment");

    let mut client = Client::builder(&token)
        .await
        .expect("Error creating Discord client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

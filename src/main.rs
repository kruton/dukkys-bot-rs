mod commands;

use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::{
        standard::macros::{group, hook},
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use tracing::Level;
use tracing::{error, info, instrument};

use commands::help::*;
use commands::request::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(help, request)]
struct General;

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected TOKEN environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .before(before)
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
        error!("Client error: {:?}", why);
    }
}

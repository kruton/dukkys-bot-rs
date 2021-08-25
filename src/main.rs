mod commands;

use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::{
        standard::macros::{group, hook},
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
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
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let game = Activity::listening("-help");
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(game), status).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_private() && msg.author.id != ctx.cache.current_user_id().await {
            if let Err(why) = msg.reply(&ctx, "Please write me in a channel!").await {
                error!("Could not reply: {:?}", why);
            }
        }
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

    // We do not support private messages.
    if msg.is_private() {
        return false;
    }

    true
}

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected TOKEN environment");

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("-").dynamic_prefix(|_, msg| {
                Box::pin(async move {
                    if msg.is_private() {
                        Some("".to_string())
                    } else {
                        None
                    }
                })
            })
        })
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

use std::time::Duration;
use serenity::{
    framework::standard::{Args, macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use tracing::error;
use tokio::time::sleep;

use crate::util::{channel::get_channel_by_name, role::get_role_by_name};

#[command]
async fn request(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let join_here = get_channel_by_name(ctx, msg.guild_id, "join-here")
        .await
        .ok_or_else(|| {
            error!("#ticket channel not found");
            "no #ticket"
        })?;

    if msg.channel_id != join_here {
        let reply = MessageBuilder::new()
            .push("Please run the command in ")
            .channel(join_here)
            .build();
        match msg.reply(ctx, reply).await {
            Ok(reply) => {
                tokio::spawn(async {
                    sleep(Duration::from_secs(10)).await;
                    reply.delete(ctx);
                });
            },
            _ => (),
        }
        return Ok(());
    }

    let minecraft_name = match args.single_quoted::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(ctx, "Please give your Minecraft Username after the command.").await?;
            return Ok(());
        }
    };

    let smp_requests = get_channel_by_name(ctx, msg.guild_id, "smp-requests")
        .await
        .ok_or_else(|| {
            error!("#ticket channel not found");
            "no #ticket"
        })?;

    let smper = get_role_by_name(ctx, msg.guild_id, "SMPer")
        .await
        .ok_or_else(|| {
            error!("@SMPer role not found");
            "no SMPer role"
        })?;

    let smp_mod = get_role_by_name(ctx, msg.guild_id, "SMP-Mod")
        .await
        .ok_or_else(|| {
            error!("@SMPer role not found");
            "no SMPer role"
        })?;
    Ok(())
}

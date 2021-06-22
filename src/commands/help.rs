use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::collections::HashMap;
use tracing::error;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| {
        error!("Could not find Guild this message was sent on");
        "no guild"
    })?;

    let channels = ctx.cache.guild_channels(guild_id).await.ok_or_else(|| {
        error!("Could not fetch channels");
        "no channels"
    })?;
    let ticket_channel = get_channel_by_name(&channels, "ticket").ok_or_else(|| {
        error!("#ticket channel not found");
        "no #ticket"
    })?;
    let question_channel = get_channel_by_name(&channels, "questions").ok_or_else(|| {
        error!("#questions channel not found");
        "no #questions"
    })?;

    let response = MessageBuilder::new()
        .push("To be whitelisted on Dukky's SMP, just type, ")
        .push_mono("-request <Minecraft Username>")
        .push(". We will whitelist you on the server shortly. ")
        .push("To join, simply connect to ")
        .push_mono("mc.hypixel.net")
        .push(" on Minecraft version 1.17 and accept the SMP invite. ")
        .push("Then type ")
        .push_mono("/smp")
        .push(" on Hypixel and select Dukky's SMP. ")
        .push("You should then shortly be warped into the server. ")
        .push("If you have any problems or questions, please open a ticket in ")
        .channel(ticket_channel)
        .push(" or ask in ")
        .channel(question_channel)
        .push(".")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        error!("Erorr sending message: {:?}", why);
    }

    Ok(())
}

fn get_channel_by_name(
    channels: &HashMap<ChannelId, GuildChannel>,
    name: &str,
) -> Option<ChannelId> {
    channels.iter().find_map(|(key, val)| {
        if val.name == name {
            Some(key.clone())
        } else {
            None
        }
    })
}

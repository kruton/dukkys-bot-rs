use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use tracing::error;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let ticket_channel = get_channel_by_name(ctx, msg.guild_id, "ticket")
        .await
        .ok_or_else(|| {
            error!("#ticket channel not found");
            "no #ticket"
        })?;
    let question_channel = get_channel_by_name(ctx, msg.guild_id, "questions")
        .await
        .ok_or_else(|| {
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

async fn get_channel_by_name(
    ctx: &Context,
    guild_id: Option<GuildId>,
    name: &str,
) -> Option<ChannelId> {
    let guild_id = guild_id.expect("Guild ID should not be empty");

    let channels = ctx
        .cache
        .guild_channels(guild_id)
        .await
        .expect("Channels are not cached");

    channels.iter().find_map(|(key, val)| {
        if val.name == name {
            Some(key.clone())
        } else {
            None
        }
    })
}

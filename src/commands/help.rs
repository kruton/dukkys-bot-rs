use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use tracing::error;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Ok(guild) = msg.guild_id.unwrap().to_partial_guild(&ctx).await {
        let channels = guild.channels(&ctx).await.unwrap();

        let channel_ticket = channels.values().find(|c| c.name == "ticket").unwrap();

        let channel_question = channels.values().find(|c| c.name == "questions").unwrap();

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
            .channel(channel_ticket)
            .push(" or ask in ")
            .channel(channel_question)
            .push(".")
            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            error!("Error sending message: {:?}", why);
        }
    };

    Ok(())
}

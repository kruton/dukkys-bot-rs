use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[command]
async fn request(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Ok(guild) = msg.guild_id.unwrap().to_partial_guild(&ctx).await {
        let channel_join_here = guild.channel_id_from_name(ctx, "join-here").await.unwrap();

        if msg.channel_id != channel_join_here {
            let reply = MessageBuilder::new()
                .push("Please run the command in ")
                .channel(channel_join_here)
                .build();

            match msg.reply(ctx, reply).await {
                Ok(reply) => {
                    let ctx1 = ctx.clone();
                    tokio::spawn(async move {
                        sleep(Duration::from_secs(10)).await;
                        reply.delete(ctx1).await
                    });
                }
                _ => (),
            }
            return Ok(());
        }

        let minecraft_name = match args.single_quoted::<String>() {
            Ok(x) => x,
            Err(_) => {
                msg.reply(
                    ctx,
                    "Please give your Minecraft Username after the command.",
                )
                .await?;
                return Ok(());
            }
        };

        let channel_smp_requests = guild
            .channel_id_from_name(ctx, "smp-requests")
            .await
            .unwrap();

        let role_smper = guild.role_by_name("SMPer").unwrap();

        let role_smp_mod = guild.role_by_name("SMP-Mod").unwrap();

        msg.reply(
            ctx,
            MessageBuilder::new()
                .mention(&msg.author)
                .push(", your request has been sent! Please be patient.")
                .build(),
        )
        .await?;

        match msg.member(ctx).await {
            Ok(mut member) => {
                let _ = member.add_role(ctx, role_smper).await;

                let _ = channel_smp_requests
                    .send_message(ctx, |m| {
                        let request_msg = MessageBuilder::new()
                            .mention(role_smp_mod)
                            .push(", ")
                            .mention(&member)
                            .push(" has sent a request! Their username is ")
                            .push_mono(minecraft_name)
                            .build();
                        m.content(request_msg);
                        m
                    })
                    .await;
            }
            Err(_) => (),
        }
    }
    Ok(())
}

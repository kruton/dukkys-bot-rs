use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
};

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push("Hi, ")
        .push_bold_safe(&msg.author.name)
        .push(". thanks.")
        .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Erorr sending message: {:?}", why);
        }

    Ok(())
}
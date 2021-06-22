use serenity::{model::prelude::*, prelude::*};

pub async fn get_channel_by_name(
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

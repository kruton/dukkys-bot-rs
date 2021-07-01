use serenity::{model::prelude::*, prelude::*};

pub async fn get_role_by_name(
    ctx: &Context,
    guild_id: Option<GuildId>,
    name: &str,
) -> Option<RoleId> {
    let guild_id = guild_id.expect("Guild ID should not be empty");

    let roles = ctx
        .cache
        .guild_roles(guild_id)
        .await
        .expect("Roles are not cached");

    roles.iter().find_map(|(key, val)| {
        if val.name == name {
            Some(key.clone())
        } else {
            None
        }
    })
}

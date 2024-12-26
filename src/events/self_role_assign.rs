use serenity::all::{Context, Reaction, ReactionType, RoleId};
use shuttle_runtime::SecretStore;
use tracing::error;

pub async fn self_role_assign(
    ctx: Context,
    reaction: Reaction,
    secrets: SecretStore,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the emoji name from the reaction
    let emoji_name = match reaction.emoji {
        ReactionType::Unicode(ref emoji) => emoji.to_string(),
        _ => return Ok(()), // Return early for non-Unicode emojis
    };
    println!("received emoji is {}", emoji_name);

    // Get role ID from secrets
    let role_id = match emoji_name {
        name if name
            == secrets
                .get("EMOJI_RUSTACEAN")
                .ok_or("Missing EMOJI_RUSTACEAN")? =>
        {
            RoleId::new(
                secrets
                    .get("ROLE_RUSTACEAN")
                    .ok_or("Missing ROLE_GAMER")?
                    .parse()?,
            )
        }
        name if name
            == secrets
                .get("EMOJI_WEB_DEV")
                .ok_or("Missing EMOJI_WEB_DEV")? =>
        {
            RoleId::new(
                secrets
                    .get("ROLE_WEB_DEV")
                    .ok_or("Missing ROLE_ARTIST")?
                    .parse()?,
            )
        }
        name if name == secrets.get("EMOJI_READER").ok_or("Missing EMOJI_READER")? => RoleId::new(
            secrets
                .get("ROLE_READER")
                .ok_or("Missing ROLE_READER")?
                .parse()?,
        ),
        _ => return Ok(()), // Return early if emoji doesn't match any role
    };

    // Get the user who reacted
    let user_id = match reaction.user_id {
        Some(id) => id,
        None => return Ok(()), // Return early
    };

    // Get the guild
    let guild_id = match reaction.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    if let Err(e) = async {
        let member = guild_id.member(&ctx.http, user_id).await?;
        if member.roles.contains(&role_id) {
            member.remove_role(&ctx.http, role_id).await?;
        } else {
            member.add_role(&ctx.http, role_id).await?;
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await
    {
        error!("Failed to add role to user: {:?}", e);
    }

    println!("your set role is {} ", role_id);
    reaction.delete(&ctx.http).await?;

    Ok(())
}

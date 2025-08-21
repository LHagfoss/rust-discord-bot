use crate::{Context, Error};
use poise::serenity_prelude::Mentionable;

#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let app_info = ctx
        .serenity_context()
        .http
        .get_current_application_info()
        .await?;
    let owner = &app_info.owner;

    let response = format!(
        "Hello! I'm a bot created by {}. If you have any questions, feel free to ask them! Gurt",
        match owner {
            Some(owner) => owner.mention().to_string(),
            None => "the bot owner".to_string(),
        }
    );

    ctx.say(response).await?;
    Ok(())
}

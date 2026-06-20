use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello").await?;
    Ok(())
}

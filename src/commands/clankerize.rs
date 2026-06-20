use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn clankerize(
    ctx: Context<'_>,
    #[description = "The text you want to clankerize"]
    text: String,                                         
) -> Result<(), Error> {
    let response = format!("# `{text} 🤖`");
    ctx.say(response).await?;
    Ok(())
}

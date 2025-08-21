mod commands;

use commands::{about::*, ping::*, time::*};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    info!("Token registered succesfully!");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), about(), time()],
            pre_command: |ctx| {
                Box::pin(async move {
                    info!("Executing command: {}", ctx.command().name);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                for command in &framework.options().commands {
                    info!("Registering command: {}", command.name);
                }
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

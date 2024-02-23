use std::env;

use error::EventError;
use log::info;
use poise::{
    builtins,
    serenity_prelude::{self as serenity, futures::TryFutureExt},
    Framework, FrameworkContext, FrameworkOptions, PrefixFrameworkOptions,
};

use serenity::{Client, GatewayIntents};

use crate::{
    common::Data,
    error::{CommandError, RuntimeError},
    module::test::test as test_command,
};

mod common;
mod database;
mod error;
mod models;
mod module;

async fn app() -> Result<(), RuntimeError> {
    let token = env::var("DISCORD_TOKEN")?;

    let db = database::create_pool().await?;
    sqlx::migrate!().run(&db).await?;

    let framework = Framework::builder()
        .setup(
            move |ctx, ready, framework: &Framework<Data, CommandError>| {
                Box::pin(async move {
                    info!("Logged in as {}", ready.user.name);
                    builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data { db })
                })
            },
        )
        .options(FrameworkOptions {
            event_handler: |_ctx, _event, _framework, _data| {
                Box::pin(
                    event_handler(_ctx, _event, _framework, _data)
                        .map_err(|e| CommandError::Event(e.into())),
                )
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("br;".into()),
                ..Default::default()
            },
            commands: vec![test_command()],
            ..Default::default()
        })
        .build();

    let client = Client::builder(&token, GatewayIntents::all())
        .framework(framework)
        .await;

    client?.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    let envy_res = dotenvy::dotenv();
    env_logger::init();
    match envy_res {
        Ok(_) => {
            info!("Loaded .env file");
        }
        Err(_) => {
            info!("No .env file found");
        }
    };

    info!("Starting server...");

    app().await
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: FrameworkContext<'_, Data, CommandError>,
    data: &Data,
) -> Result<(), EventError> {
    match event {
        serenity::FullEvent::GuildCreate { guild, is_new } => {
            module::triage::handle_guild_create(
                models::EventInfo {
                    ctx: ctx.clone(),
                    event: event.clone(),
                    framework,
                    data: data.clone(),
                },
                guild,
                is_new,
            )
            .await?;
        }
        _ => {}
    }

    Ok(())
}

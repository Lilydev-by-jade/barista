use std::env;

use error::EventError;
use log::{debug, error, info};
use poise::{
    builtins,
    samples::on_error,
    serenity_prelude::{self as serenity, futures::TryFutureExt, CreateEmbed},
    CreateReply, Framework, FrameworkContext, FrameworkError, FrameworkOptions,
    PrefixFrameworkOptions,
};

use serenity::{Client, GatewayIntents};

use crate::{
    common::Data,
    error::{CommandError, RuntimeError},
    module::config::config as config_command,
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
                    event_handler(_ctx, _event, _framework, _data).map_err(CommandError::Event),
                )
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("br;".into()),
                ..Default::default()
            },
            commands: vec![config_command(), test_command()],
            on_error: |er| {
                Box::pin(async move {
                    if let Err(e) = handle_error(er).await {
                        error!("Error while handling error: {}", e);
                    }
                })
            },
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
            module::triage::guild_create::handle_guild_create(
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
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            module::triage::member_join::handle_triage_request(
                models::EventInfo {
                    ctx: ctx.clone(),
                    event: event.clone(),
                    framework,
                    data: data.clone(),
                },
                new_member,
            )
            .await?;
        }
        serenity::FullEvent::InteractionCreate { interaction: _ } => {
            // TODO: handle interaction with triage buttons by getting id and type
            // from button id
        }
        _ => {}
    }

    Ok(())
}

pub async fn handle_error(
    error: FrameworkError<'_, Data, CommandError>,
) -> Result<(), RuntimeError> {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .color(0xff3333)
                            .title(format!("⁉️ Error running command `{}`", ctx.command().name))
                            .description(format!("{}", error)),
                    )
                    .ephemeral(true),
            )
            .await?;
            debug!("Error running command: {}", error);
        }
        error => {
            if let Err(err) = on_error(error).await {
                error!("Error in on_error handler: {:?}", err);
            }
        }
    }
    Ok(())
}

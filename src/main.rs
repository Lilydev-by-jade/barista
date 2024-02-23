use std::env;

use log::info;
use poise::{
    builtins,
    serenity_prelude::{Client, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use crate::{
    common::Data,
    error::{CommandError, RuntimeError},
    module::test::test as test_command,
};

mod common;
mod database;
mod error;
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
            // event_handler: |_ctx, _event, _framework, _data| {
            //     Box::pin(event_handler(_ctx, _event, _framework, _data))
            // },
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

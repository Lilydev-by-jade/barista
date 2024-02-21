use std::env;

use log::info;
use poise::{
    builtins,
    serenity_prelude::{Client, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use crate::{
    error::{CommandError, RuntimeError},
    module::test::test as test_command,
};

mod common;
mod error;
mod module;

async fn app() -> Result<(), RuntimeError> {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            panic!(
                "Failed to get environment variable `DISCORD_TOKEN`: {}",
                err
            )
        }
    };

    let framework = Framework::builder()
        .setup(move |ctx, ready, framework: &Framework<(), CommandError>| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
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

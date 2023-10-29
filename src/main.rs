use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use poise::serenity_prelude as serenity;
use poise::{Event, Framework, FrameworkContext, FrameworkOptions, PrefixFrameworkOptions};
use sqlx::PgPool;

mod command;
mod database;
mod error;
mod models;


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    pub db: PgPool
}

async fn app() -> Result<(), Error> {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            panic!("Failed to get environment variable `DISCORD_TOKEN`: {}", err)
        }
    };

    let framework_options = FrameworkOptions {
        event_handler: |_ctx, _event, _framework, _data| {
            Box::pin(event_handler(_ctx, _event, _framework, _data))
        },
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("br;".into()),
            ..Default::default()
        },
        commands: vec![
            command::config()
        ],
        ..Default::default()
    };

    let db = database::create_pool().await?;
    sqlx::migrate!().run(&db).await?;

    Framework::builder()
        .token(token)
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as user: {}", ready.user.name);
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands
                ).await?;
                Ok(Data {
                    db
                })
            })
        })
        .options(framework_options)
        .intents(
            // serenity::GatewayIntents::default()
            //     | serenity::GatewayIntents::GUILD_MEMBERS
            //     | serenity::GatewayIntents::
            serenity::GatewayIntents::all()
        )
        .run().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match dotenvy::dotenv() {
        Ok(_) => {
            info!("Loaded variables from .env!")
        },
        Err(_) => {
            info!(".env file not present.")
        }
    }

    if let Err(err) = app().await {
        error!("{}", err);
        std::process::exit(1);
    }
}


async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    framework: FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        Event::GuildCreate { guild, is_new } => {
            match sqlx::query!(
                r#"
                INSERT INTO "triage" (guild_id, enabled)
                VALUES ($1, $2)
                "#,
                *guild.id.as_u64() as i64, false
            ).execute(&data.db).await {
                Ok(_) => {
                    info!("Joined new guild!");
                    println!("Joined new guild!");
                },
                Err(_) => {
                    info!("Joined guild! Guild already present in database.");
                    println!("Joined guild! Guild already present in database.");
                }
            };

            println!("is thing connected: {}", !data.db.is_closed());

            println!("did thing");
        }
        Event::GuildMemberAddition { new_member } => {
            println!("New member: {}", &new_member.user.name)
        }
        _ => {}
    }
    Ok(())
}

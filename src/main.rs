use std::env;
use anyhow::anyhow;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use poise::serenity_prelude as serenity;
use poise::{Event, Framework, FrameworkContext, FrameworkOptions, PrefixFrameworkOptions};
use poise::serenity_prelude::{ButtonStyle, RoleId};
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
            let guild_id = *new_member.guild_id.as_u64() as i64;

            let triage_conf = sqlx::query_as!(
                models::triage::TriageDb,
                r#"
                SELECT * FROM "triage" WHERE "guild_id" = $1
                "#, guild_id
            ).fetch_one(&data.db).await?;

            let approve_button_id = uuid::Uuid::new_v4();
            let kick_button_id = uuid::Uuid::new_v4();
            let ban_button_id = uuid::Uuid::new_v4();

            if triage_conf.enabled {
                if triage_conf.moderator_channel_id.is_none() || triage_conf.member_role_id.is_none() {
                    return Err(
                        Error::try_from(anyhow!(
                            "Attempted to send triage request with null \
                            `moderator_channel` or `member_role`"
                        )).unwrap()
                    );
                } else {
                    let moderator_channel = ctx.http.get_channel(
                        triage_conf.moderator_channel_id.unwrap() as u64
                    ).await?;
                    let member_role = match RoleId(triage_conf.member_role_id.unwrap() as u64)
                        .to_role_cached(&ctx.cache) {
                            Some(role) => role,
                            None => return Err(
                                Error::try_from(anyhow!(
                                    "Failed to get member role"
                                )).unwrap()
                            )
                        };

                    moderator_channel.id().send_message(ctx, |m| {
                        m.embed(|e| {
                            e.author(|author| {
                                match new_member.avatar.clone() {
                                    Some(avatar_url) => author.icon_url(avatar_url),
                                    None => author.icon_url(new_member.face())
                                }.name(&new_member.user.name)
                            })
                                .color(0xebc034)
                                .description(
                                    format!("User **{}** requested to join!", &new_member.user.name)
                                )
                                .footer(|f| {
                                    f.text(
                                        format!(
                                            "Account created {} UTC",
                                            new_member.user.created_at()
                                                .format("%b, %e %Y %r"),
                                        )
                                    )
                                })
                        }).components(|comp| {
                            comp.create_action_row(|ar| {
                                ar.create_button(|button| {
                                    button.style(ButtonStyle::Success)
                                        .label("Approve User")
                                        .custom_id(approve_button_id)
                                }).create_button(|button| {
                                    button.style(ButtonStyle::Danger)
                                        .label("Kick User")
                                        .custom_id(kick_button_id)
                                }).create_button(|button| {
                                    button.style(ButtonStyle::Danger)
                                        .label("Ban User")
                                        .custom_id(ban_button_id)
                                })
                            })
                        })
                    }).await?;

                }

            }


        }
        _ => {}
    }
    Ok(())
}

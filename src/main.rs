use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use serenity::{
    async_trait,
    prelude::*,
    framework::StandardFramework,
    model::guild::Member
};
use serenity::model::guild::Guild;
use sqlx::{Pool, Postgres};

mod database;


struct Handler {
    db: Pool<Postgres>
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        println!("I joined a new guild!");
        println!("Guild Name: {}", guild.name);
        println!("Guild ID: {}", guild.id);
        println!("Guild New: {}", is_new);
    }

    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        debug!("{} joined!", new_member.user.name);
        println!("{} joined!", new_member.user.name);
    }
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    match dotenvy::dotenv() {
        Ok(_) => {
            info!("Loaded variables from .env!")
        },
        Err(_) => {
            info!(".env file not present.")
        }
    }

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            panic!("Failed to get environment variable `DISCORD_TOKEN`: {}", err)
        }
    };
    let intents = GatewayIntents::all();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("br;"));

    let db = database::create_pool().await?;

    let event_handler = Handler {
        db: db.clone()
    };

    let mut client = Client::builder(token, intents)
        .event_handler(event_handler)
        .framework(framework)
        .await?;

    if let Err(err) = client.start().await {
        error!("Error while running client: {}", err)
    };

    Ok(())
}

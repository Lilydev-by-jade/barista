use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use serenity::{
    async_trait,
    prelude::*
};
use serenity::framework::StandardFramework;
use serenity::model::guild::Member;

struct JoinHandler;

#[async_trait]
impl EventHandler for JoinHandler {
    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        debug!("{} joined!", new_member.user.name);
        println!("{} joined!", new_member.user.name);
    }
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

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            panic!("Failed to get environment variable `DISCORD_TOKEN`: {}", err)
        }
    };
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("br;"));

    let mut client = Client::builder(token, intents)
        .event_handler(JoinHandler)
        .framework(framework)
        .await
        .expect("Failed to create client.");

    if let Err(err) = client.start().await {
        error!("Error while running client: {}", err)
    }
}

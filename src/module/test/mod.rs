use crate::{common::Context, error::CommandError};

pub mod echo;

#[poise::command(slash_command, prefix_command, subcommands("echo::echo"))]
pub async fn test(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

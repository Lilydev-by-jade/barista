use crate::{common::Context, error::CommandError};

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("crate::module::triage::command::triage")
)]
pub async fn config(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

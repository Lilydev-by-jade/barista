use crate::{common::Context, error::CommandError};

#[poise::command(slash_command, prefix_command, subcommands("triage"))]
pub async fn config(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "crate::module::triage::command::enable",
        "crate::module::triage::command::disable"
    )
)]
pub async fn triage(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

#[poise::command(slash_command, prefix_command, subcommands())]
pub async fn auto_assign_role(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

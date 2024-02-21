use crate::{common::Context, error::CommandError};

#[poise::command(slash_command, prefix_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message to echo"] message: String,
) -> Result<(), CommandError> {
    ctx.say(message).await?;
    Ok(())
}

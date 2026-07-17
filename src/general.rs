use crate::{Context, Error};

/// Ping the bot to check latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Pong!\nThat took **{}ms**",
        ctx.ping().await.as_millis()
    ))
    .await?;
    Ok(())
}

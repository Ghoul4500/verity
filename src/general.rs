use crate::{Context, Error};

/// Ping the bot to check latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await;

    if ping.is_zero() {
        ctx.say("Ping not ready yet. Try again in a few seconds")
            .await?;
    } else {
        ctx.say(format!("Pong!\nThat took **{}ms**", ping.as_millis()))
            .await?;
    }
    Ok(())
}

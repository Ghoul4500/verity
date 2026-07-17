use crate::{Context, Error};

use chrono::Utc;
use indoc::formatdoc;
use poise::serenity_prelude as serenity;
use serenity::builder;

/// Ping the bot to check latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await;
    let message_time = *ctx.created_at();

    let embed = builder::CreateEmbed::new()
        .title("Pong!")
        .color(serenity::Color::BLUE)
        .description(formatdoc!(
            "
            ```rs
            Websocket Latency:      {}
            Message Latency:        {}ms
            ```
            ",
            match ping.as_millis() {
                0 => "N/A".to_string(),
                _ => format!("{}ms", ping.as_millis()),
            },
            (Utc::now() - message_time).num_milliseconds()
        ))
        .footer(
            builder::CreateEmbedFooter::new(format!("Requested by {}", ctx.author().name.clone()))
                .icon_url(ctx.author().face()),
        );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

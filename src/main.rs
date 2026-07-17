use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

pub mod general;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let prefix = std::env::var("PREFIX").unwrap_or("!".to_string());

    let options = poise::FrameworkOptions {
        commands: vec![general::ping()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(prefix),
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let intents = serenity::GatewayIntents::all();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

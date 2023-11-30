mod constants;

use std::env;

use serenity::all::{Client, Context, EventHandler, GatewayIntents, Ready, ChannelType};
use serenity::async_trait;
use tokio::time::{interval, Duration};

use crate::constants::{CHECK_INTERVAL, GUILD_ID, MENTION_REGEX, NOTIFY_DURATION, NOTIFY_TEXT, TIMES_CATEGORY_ID};

#[inline]
fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
    intents.insert(GatewayIntents::GUILDS);
    intents.insert(GatewayIntents::GUILD_MESSAGES);
    intents
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let mut interval = interval(Duration::from_secs(CHECK_INTERVAL));

        loop {
            interval.tick().await;
            log::info!("Checking for new times...");
            let channels = GUILD_ID.channels(&ctx.http).await.unwrap();
            let times = channels
                .values()
                .filter(|c| c.parent_id == Some(*TIMES_CATEGORY_ID) && c.kind == ChannelType::Text)
                .collect::<Vec<_>>();

            for time in times {
                log::info!("Checking {}...", time.name);
                let last_message_id = time.last_message_id;
                if last_message_id.is_none() {
                    log::debug!("{} has no messages", time.name);
                }
                let last_message_id = last_message_id.unwrap();
                //let last_message = time.message(&ctx.http, last_message_id).await.unwrap();
                //let last_message_time = last_message.timestamp.timestamp();
                let last_message_time = (rustflakes::Snowflake::from(last_message_id.get()).timestamp as f64 / 1000.0).floor() as i64;
                let now = chrono::Utc::now().timestamp();
                let duration_sec = (NOTIFY_DURATION * 60) as i64;

                let topic = time.topic.as_ref().unwrap();
                let assignee_id = MENTION_REGEX.captures(topic).unwrap().get(1).unwrap().as_str();

                if now - last_message_time > duration_sec {
                    log::info!("Notifying {}...", time.name);
                    time.say(&ctx.http, format!("<@{}> {}", assignee_id, NOTIFY_TEXT)).await.unwrap();
                }
            }
            log::info!("Done!");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::builder()
        .filter_module("times_notifier", {
            if cfg!(debug_assertions) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            }
        })
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, get_intents())
        .event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}

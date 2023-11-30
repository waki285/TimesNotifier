use once_cell::sync::Lazy;
use serenity::all::{ChannelId, GuildId};
use regex::Regex;

// secs
pub static CHECK_INTERVAL: u64 = 600;
// minutes
pub static NOTIFY_DURATION: u64 = 4320;

pub static NOTIFY_TEXT: &str = "進捗どうですか？";
pub static GUILD_ID: Lazy<GuildId> = Lazy::new(|| GuildId::new(1173435345354887198));
pub static TIMES_CATEGORY_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::new(1173441281955987476));

pub static MENTION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"<@!?(\d+)>").unwrap());
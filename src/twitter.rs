use anyhow::{Ok, Result};
use egg_mode::tweet::DraftTweet;
use egg_mode::{auth, Token};
use once_cell::sync::Lazy;
use std::env::var;

use crate::structs::TwitterConfig;

impl TwitterConfig {
    fn new() -> TwitterConfig {
        TwitterConfig {
            access_key: var("TW_ACCESS_KEY").unwrap(),
            consumer_key: var("TW_CONSUMER_KEY").unwrap(),
            access_secret: var("TW_ACCESS_SECRET").unwrap(),
            consumer_secret: var("TW_CONSUMER_SECRET").unwrap(),
        }
    }

    fn create_token(&self) -> Token {
        let consumer =
            egg_mode::KeyPair::new(self.consumer_key.clone(), self.consumer_secret.clone());

        let access = egg_mode::KeyPair::new(self.access_key.clone(), self.access_secret.clone());
        let value = egg_mode::Token::Access { consumer, access };

        value
    }
}

static CLIENT: Lazy<TwitterConfig> = Lazy::new(TwitterConfig::new);
static TOKEN: Lazy<Token> = Lazy::new(|| CLIENT.create_token());

pub async fn make_tweet(content: String) -> Result<()> {
    if auth::verify_tokens(&TOKEN).await.is_ok() {
        //
    }

    DraftTweet::new(content).send(&TOKEN).await?;

    Ok(())
}

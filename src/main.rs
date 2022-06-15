use anyhow::Result;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
struct Beach {
    name: String,
    status: i8,
}

struct SupabaseConfig {
    url: String,
    anon_public: String,
}

struct TwitterConfig {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

// const CODES: [i8; 5] = [0, 1, 2, 3, 4];
const PARAMS: [(&str, &str); 2] = [("status", "eq.1"), ("select", "name,status")];

const ASCII_ART: &str = r"
_\/_                 |                _\/_
/o\\             \       /            //o\
 |                 .---.                |
_|_______     --  /     \  --     ______|__
         `~^~^~^~^~^~^~^~^~^~^~^~`

";

#[tokio::main]
async fn main() {
    println!("{}", ASCII_ART);

    // loop{}

    let supabase = SupabaseConfig {
        anon_public: var("SUPABASE_KEY").unwrap(),
        url: var("SUPABASE_URL").unwrap(),
    };

    let twitter = TwitterConfig {
        consumer_key: var("TW_CONSUMER_KEY").unwrap(),
        consumer_secret: var("TW_CONSUMER_SECRET").unwrap(),
        access_key: var("TW_ACCESS_KEY").unwrap(),
        access_secret: var("TW_ACCESS_SECRET").unwrap(),
    };

    let mut owned_str = "Bearer ".to_owned();
    owned_str.push_str(supabase.anon_public.as_str());

    let mut headers = HeaderMap::new();

    headers.insert("apikey", supabase.anon_public.parse().unwrap());
    headers.insert(AUTHORIZATION, owned_str.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let req = retrieve_data_from_rest(supabase, headers).await;
    let mut beaches = req.unwrap();
    beaches.shuffle(&mut thread_rng());

    let mut record = "🌊 Cancún 🌊\n\n".to_string();

    for beach in &beaches[0..3] {
        let (_color, emoji) = get_status_color(&beach.status);
        let text = beach.name.to_uppercase() + " " + emoji + "\n";

        record.push_str(text.as_str());
    }

    println!("{}", record);

    match make_tweet(twitter, record).await {
        Ok(()) => println!("{}", "Posted"),
        _ => println!("{}", "Failed"),
    }
}
fn get_status_color(code: &i8) -> (&'static str, &'static str) {
    match code {
        4 => ("red", "🔴"),
        3 | 2 => ("yellow", "🟡"),
        1 | 0 => ("green", "🟢"),
        _ => ("", ""),
    }
}

async fn make_tweet(config: TwitterConfig, content: String) -> Result<()> {
    let consumer = egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret);
    let access = egg_mode::KeyPair::new(config.access_key, config.access_secret);
    let token = egg_mode::Token::Access { consumer, access };

    if egg_mode::auth::verify_tokens(&token).await.is_ok() {
        //
    }

    let tweet = egg_mode::tweet::DraftTweet::new(content);
    tweet.send(&token).await?;

    Ok(())
}

async fn retrieve_data_from_rest(
    config: SupabaseConfig,
    headers: HeaderMap,
) -> Result<Vec<Beach>, reqwest::Error> {
    let client = reqwest::Client::new();

    let resp = client
        .get(config.url + "/rest/v1/beaches")
        .query(&PARAMS)
        .headers(headers)
        .send()
        .await?;

    let data = resp.json().await?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::get_status_color;

    #[test]
    fn test_status_color() {
        let sample = [
            (0, ("green", "")),
            (1, ("green", "")),
            (2, ("yellow", "")),
            (3, ("yellow", "")),
            (4, ("red", "")),
            (5, ("", "")),
        ];

        for item in sample {
            let (code, expected) = item;
            assert_eq!(get_status_color(&code), expected)
        }
    }
}

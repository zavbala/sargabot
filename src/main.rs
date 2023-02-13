use rand::prelude::SliceRandom;
use rand::thread_rng;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::env::var;

mod utils;
use utils::{get_status_color, retrieve_data_from_rest};

mod twitter;
use twitter::make_tweet;

mod structs;
use structs::SupabaseConfig;

mod constant;
use constant::{ASCII_ART, MESSAGE, WEBSITE};

#[tokio::main]
async fn main() {
    println!("{}", ASCII_ART);

    let supabase = SupabaseConfig {
        url: var("SUPABASE_URL").unwrap(),
        anon_public: var("SUPABASE_KEY").unwrap(),
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
        let text = emoji.to_owned() + " " + beach.name.to_uppercase().as_str() + "\n";

        record.push_str(text.as_str());
    }

    record.push_str("\n");
    record.push_str(MESSAGE);
    record.push_str(WEBSITE);

    println!("{}", record);

    match make_tweet(record).await {
        Ok(()) => println!("{}", "\nSuccess ✅"),
        _ => println!("{}", "\nFail ❌"),
    }
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

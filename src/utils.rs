use crate::constant::PARAMS;
use crate::structs::{Beach, SupabaseConfig};
use reqwest::header::HeaderMap;

pub async fn retrieve_data_from_rest(
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

pub fn get_status_color(code: &i8) -> (&'static str, &'static str) {
    match code {
        4 => ("red", "🔴"),
        3 | 2 => ("yellow", "🟡"),
        1 | 0 => ("green", "🟢"),
        _ => ("", ""),
    }
}

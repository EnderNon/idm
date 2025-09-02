use crate::json::jsonstruct::*;
use leptos::leptos_dom::logging::console_log;
use reqwasm::http::Request;
use std::collections::HashMap;

/// Fetches all data from wynntils. See:
/// - https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json
/// - https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json
/// - https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json
pub async fn fetch_all(
    url_id_keys: &str,
    url_shiny_stats: &str,
    url_gear: &str,
) -> Result<DlJsons, reqwasm::Error> {
    let id_keys = fetch_id_keys(url_id_keys).await?;
    let shiny_stats = fetch_shiny_stats(url_shiny_stats).await?;
    let gear = fetch_gear(url_gear).await?;
    console_log(format!("1\n{gear:?}").as_str());
    Ok(DlJsons {
        id_keys,
        shiny_stats,
        gear,
    })
}
/// Individual function to fetch id_keys.json.  
/// See https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json
pub async fn fetch_id_keys(url: &str) -> Result<HashMap<String, u8>, reqwasm::Error> {
    let resp = Request::get(url).send().await?;
    let id_keys: HashMap<String, u8> = resp.json().await?;
    Ok(id_keys)
}
/// Individual function to fetch shiny_stats.json.
/// See https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json
pub async fn fetch_shiny_stats(url: &str) -> Result<Vec<Shinystruct>, reqwasm::Error> {
    let resp = Request::get(url).send().await?;
    let id_keys: Vec<Shinystruct> = resp.json().await?;
    Ok(id_keys)
}
/// Individual function to fetch gear.json
/// See https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json
pub async fn fetch_gear(url: &str) -> Result<HashMap<String, GearJsonItem>, reqwasm::Error> {
    let resp = Request::get(url).send().await?;
    let id_keys: HashMap<String, GearJsonItem> = resp.json().await?;
    Ok(id_keys)
}

use chrono::{
    DateTime, Utc,
    serde::{ts_milliseconds, ts_seconds},
};
use std::path::Path;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct NoteEntry {
    #[serde(rename = "Note ID")]
    pub note_id: u64,
    #[serde(rename = "Created at", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "Tweet ID")]
    pub tweet_id: Option<u64>,
    #[serde(rename = "User ID")]
    pub user_id: Option<u64>,
    #[serde(rename = "Misleading")]
    pub misleading: Option<bool>,
    #[serde(rename = "Helpful")]
    pub helpful: Option<bool>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub id: u64,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub screen_name: String,
    pub follower_count: usize,
}

pub fn accounts<P: AsRef<Path>>(path: P) -> Result<Vec<Account>, csv::Error> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .deserialize()
        .collect()
}

pub fn account_ids<P: AsRef<Path>>(path: P) -> Result<Vec<u64>, csv::Error> {
    let accounts = accounts(path)?;

    Ok(accounts.into_iter().map(|account| account.id).collect())
}

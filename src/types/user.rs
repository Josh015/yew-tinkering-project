use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use super::Note;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Option<u64>,
    birth_date: DateTime<Utc>,
    name: String,
    avatar: String,
    bio: String,
    gender: String,
    notes: Vec<Note>,
}

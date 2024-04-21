use crate::task::Task;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeRecord {
    /// Time record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Time recorded in seconds
    pub time: i64,
    /// User ID
    pub user: i64,
    /// Date
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_invoiced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

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

impl TimeRecord {
    pub fn for_adding(date: NaiveDate, user: i64, time: i64, comment: Option<String>) -> Self {
        TimeRecord {
            id: None,
            time,
            user,
            date,
            task: None,
            is_locked: None,
            is_invoiced: None,
            comment,
        }
    }
}

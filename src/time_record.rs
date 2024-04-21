use crate::task::Task;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TimeRecord {
    /// Time record ID
    pub id: i64,
    /// Time recorded in seconds
    pub time: i64,
    /// User ID
    pub user: i64,
    /// Date
    pub date: NaiveDate,
    pub task: Option<Task>,
    pub is_locked: Option<bool>,
    pub is_invoiced: Option<bool>,
    pub comment: Option<String>,
}

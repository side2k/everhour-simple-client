use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub projects: Vec<serde_json::Value>,
    /// Section ID
    pub section: Option<f64>,
    pub labels: Option<Vec<serde_json::Value>>,
    pub position: Option<f64>,
    pub description: Option<String>,
    /// Format: Y-m-d H:i:s
    pub due_at: Option<String>,
    pub status: Option<Status>,
    pub time: Option<TaskTime>,
    pub estimate: Option<TaskEstimate>,
    /// Custom attributes from integration
    pub attributes: Option<serde_json::Value>,
    /// Custom metrics from integration
    pub metrics: Option<serde_json::Value>,
    pub unbillable: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskTime {
    /// Total task time in seconds
    pub total: i64,
    pub users: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskEstimate {
    /// Total task estimate in seconds
    pub total: i64,
    #[serde(rename = "type")]
    pub r#type: TaskEstimateType,
    pub users: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TaskEstimateType {
    #[serde(rename = "overall")]
    Overall,
    #[serde(rename = "users")]
    Users,
}

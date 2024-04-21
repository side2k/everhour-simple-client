use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "workspaceName", skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    /// Client ID
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    /// Project Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProjectType>,
    pub favorite: Option<bool>,
    /// List of assigned user IDs
    pub users: Option<Vec<serde_json::Value>>,
    pub billing: Option<ProjectBillingBilling>,
    pub rate: Option<ProjectBillingRate>,
    pub budget: Option<ProjectBillingBudget>,
}

#[derive(Deserialize, Debug)]
pub enum ProjectType {
    #[serde(rename = "board")]
    Board,
    #[serde(rename = "list")]
    List,
}

#[derive(Deserialize, Debug)]
pub struct ProjectBilling {
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none")]
    pub billing: Option<ProjectBillingBilling>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<ProjectBillingRate>,
    #[serde(rename = "budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<ProjectBillingBudget>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectBillingBilling {
    /// Project Type
    #[serde(rename = "type")]
    pub r#type: ProjectBillingBillingType,
    /// Project fixed fee in cents (e.g. 10000 means $100.00). Available only for `fixed_fee` type.
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub enum ProjectBillingBillingType {
    #[serde(rename = "non_billable")]
    NonBillable,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "fixed_fee")]
    FixedFee,
}

#[derive(Deserialize, Debug)]
pub struct ProjectBillingRate {
    /// Project Type
    #[serde(rename = "type")]
    pub r#type: ProjectBillingRateType,
    /// Flat-rate in cents (e.g. 10000 means $100.00). Available only for `project_rate` type.
    pub rate: Option<i64>,
    /// Override user rates. Available only for `user_rate` type.
    #[serde(rename = "userRateOverrides")]
    pub user_rate_overrides: serde_json::Value,
    /// Override user cost rates. Available only for `user_cost` type.
    #[serde(rename = "userCostOverrides")]
    pub user_cost_overrides: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub enum ProjectBillingRateType {
    #[serde(rename = "project_rate")]
    ProjectRate,
    #[serde(rename = "user_rate")]
    UserRate,
    #[serde(rename = "user_cost")]
    UserCost,
}

#[derive(Deserialize, Debug)]
pub struct ProjectBillingBudget {
    /// Budget Type
    #[serde(rename = "type")]
    pub r#type: ProjectBillingBudgetType,
    /// Budget value in cents (for money) or seconds (for time)
    #[serde(rename = "budget")]
    pub budget: f64,
    /// [readonly] Current budget usage in cents (for money) or seconds (for time)
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// [readonly] Budget progress belongs to time tracking
    #[serde(rename = "timeProgress", skip_serializing_if = "Option::is_none")]
    pub time_progress: Option<f64>,
    /// [readonly] Budget progress belongs to expeses
    #[serde(rename = "expenseProgress", skip_serializing_if = "Option::is_none")]
    pub expense_progress: Option<f64>,
    /// Budget periodicity (overall, monthly, weekly, daily)
    #[serde(rename = "period")]
    pub period: ProjectBillingBudgetPeriod,
    /// Start budget from (available only for non-recurrent budgets)
    #[serde(rename = "appliedFrom", skip_serializing_if = "Option::is_none")]
    pub applied_from: Option<String>,
    /// Disallow overbudget
    #[serde(rename = "disallowOverbudget", skip_serializing_if = "Option::is_none")]
    pub disallow_overbudget: Option<bool>,
    /// Exclude non-billable time
    #[serde(
        rename = "excludeUnbillableTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_unbillable_time: Option<bool>,
    /// Exclude expenses
    #[serde(rename = "excludeExpenses", skip_serializing_if = "Option::is_none")]
    pub exclude_expenses: Option<bool>,
    /// Set true if all team members should see budget and its progress
    #[serde(rename = "showToUsers", skip_serializing_if = "Option::is_none")]
    pub show_to_users: Option<bool>,
    /// Email admins when threshold reached. Threshold is percentage: 1 - 100.
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub enum ProjectBillingBudgetType {
    #[serde(rename = "money")]
    Money,
    #[serde(rename = "time")]
    Time,
}

#[derive(Deserialize, Debug)]
pub enum ProjectBillingBudgetPeriod {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "daily")]
    Daily,
}

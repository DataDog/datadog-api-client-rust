// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryBody {
    /// The total account usage.
    #[serde(rename = "account_billable_usage", skip_serializing_if = "Option::is_none")]
    pub account_billable_usage: i64,
    /// Elapsed usage hours for some billable product.
    #[serde(rename = "elapsed_usage_hours", skip_serializing_if = "Option::is_none")]
    pub elapsed_usage_hours: i64,
    /// The first billable hour for the org.
    #[serde(rename = "first_billable_usage_hour", skip_serializing_if = "Option::is_none")]
    pub first_billable_usage_hour: String,
    /// The last billable hour for the org.
    #[serde(rename = "last_billable_usage_hour", skip_serializing_if = "Option::is_none")]
    pub last_billable_usage_hour: String,
    /// The number of units used within the billable timeframe.
    #[serde(rename = "org_billable_usage", skip_serializing_if = "Option::is_none")]
    pub org_billable_usage: i64,
    /// The percentage of account usage the org represents.
    #[serde(rename = "percentage_in_account", skip_serializing_if = "Option::is_none")]
    pub percentage_in_account: f64,
    /// Units pertaining to the usage.
    #[serde(rename = "usage_unit", skip_serializing_if = "Option::is_none")]
    pub usage_unit: String,
}


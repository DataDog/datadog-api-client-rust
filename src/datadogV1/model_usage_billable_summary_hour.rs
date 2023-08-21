// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryHour {
    /// The billing plan.
    #[serde(rename = "billing_plan", skip_serializing_if = "Option::is_none")]
    pub billing_plan: String,
    /// Shows the last date of usage.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: String,
    /// The number of organizations.
    #[serde(rename = "num_orgs", skip_serializing_if = "Option::is_none")]
    pub num_orgs: i64,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Shows usage aggregation for a billing period.
    #[serde(rename = "ratio_in_month", skip_serializing_if = "Option::is_none")]
    pub ratio_in_month: f64,
    /// The region of the organization.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// Shows the first date of usage.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: String,
    /// Response with aggregated usage types.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: UsageBillableSummaryKeys,
}


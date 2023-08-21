// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostByOrgAttributes {
    /// List of charges data reported for the requested month.
    #[serde(rename = "charges", skip_serializing_if = "Option::is_none")]
    pub charges: Vec<ChargebackBreakdown>,
    /// The month requested.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: String,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// The total cost of products for the month.
    #[serde(rename = "total_cost", skip_serializing_if = "Option::is_none")]
    pub total_cost: f64,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageAttributes {
    /// List of the measured usage values for the product family for the org for the time period.
    #[serde(rename = "measurements", skip_serializing_if = "Option::is_none")]
    pub measurements: Vec<HourlyUsageMeasurement>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The product for which usage is being reported.
    #[serde(rename = "product_family", skip_serializing_if = "Option::is_none")]
    pub product_family: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: String,
}


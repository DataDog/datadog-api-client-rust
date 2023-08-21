// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsage {
    /// Attributes of hourly usage for a product family for an org for a time period.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: HourlyUsageAttributes,
    /// Unique ID of the response.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Type of usage data.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: UsageTimeSeriesType,
}


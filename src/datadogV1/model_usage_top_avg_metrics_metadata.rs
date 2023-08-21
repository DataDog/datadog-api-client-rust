// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsMetadata {
    /// The day value from the user request that contains the returned usage data. (If day was used the request)
    #[serde(rename = "day", skip_serializing_if = "Option::is_none")]
    pub day: String,
    /// The month value from the user request that contains the returned usage data. (If month was used the request)
    #[serde(rename = "month", skip_serializing_if = "Option::is_none")]
    pub month: String,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: UsageTopAvgMetricsPagination,
}


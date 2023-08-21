// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeseriesHour {
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// Contains the number of custom metrics that are inputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_input_timeseries", skip_serializing_if = "Option::is_none")]
    pub num_custom_input_timeseries: i64,
    /// Contains the number of custom metrics that are outputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_output_timeseries", skip_serializing_if = "Option::is_none")]
    pub num_custom_output_timeseries: i64,
    /// Contains sum of non-aggregation custom metrics and custom metrics that are outputs for aggregations.
    #[serde(rename = "num_custom_timeseries", skip_serializing_if = "Option::is_none")]
    pub num_custom_timeseries: i64,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
}


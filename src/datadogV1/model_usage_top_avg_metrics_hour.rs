// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsHour {
    /// Average number of timeseries per hour in which the metric occurs.
    #[serde(rename = "avg_metric_hour", skip_serializing_if = "Option::is_none")]
    pub avg_metric_hour: i64,
    /// Maximum number of timeseries per hour in which the metric occurs.
    #[serde(rename = "max_metric_hour", skip_serializing_if = "Option::is_none")]
    pub max_metric_hour: i64,
    /// Contains the metric category.
    #[serde(rename = "metric_category", skip_serializing_if = "Option::is_none")]
    pub metric_category: UsageMetricCategory,
    /// Contains the custom metric name.
    #[serde(rename = "metric_name", skip_serializing_if = "Option::is_none")]
    pub metric_name: String,
}


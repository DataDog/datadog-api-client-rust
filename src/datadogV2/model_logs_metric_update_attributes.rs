// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricUpdateAttributes {
    /// The compute rule to compute the log-based metric.
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: LogsMetricUpdateCompute,
    /// The log-based metric filter. Logs matching this filter will be aggregated in this metric.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: LogsMetricFilter,
    /// The rules for the group by.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<LogsMetricGroupBy>,
}


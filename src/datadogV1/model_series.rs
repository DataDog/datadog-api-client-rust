// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Series {
    /// The name of the host that produced the metric.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// If the type of the metric is rate or count, define the corresponding interval.
    #[serde(rename = "interval", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub interval: Option<Int64>,
    /// The name of the timeseries.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Points relating to a metric. All points must be tuples with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Vec<Vec<*f64>>,
    /// A list of tags associated with the metric.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The type of the metric. Valid types are "",`count`, `gauge`, and `rate`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}


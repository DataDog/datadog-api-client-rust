// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionPointsSeries {
    /// The name of the host that produced the distribution point metric.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// The name of the distribution points metric.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Points relating to the distribution point metric. All points must be tuples with timestamp and a list of values (cannot be a string). Timestamps should be in POSIX time in seconds.
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Vec<Vec<DistributionPointItem>>,
    /// A list of tags associated with the distribution point metric.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The type of the distribution point.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: DistributionPointsType,
}


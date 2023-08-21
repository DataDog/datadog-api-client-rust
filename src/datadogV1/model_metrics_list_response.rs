// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsListResponse {
    /// Time when the metrics were active, seconds since the Unix epoch.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// List of metric names.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Vec<String>,
}


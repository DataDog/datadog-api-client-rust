// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMetrics {
    /// The percent of CPU used (everything but idle).
    #[serde(rename = "cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: f64,
    /// The percent of CPU spent waiting on the IO (not reported for all platforms).
    #[serde(rename = "iowait", skip_serializing_if = "Option::is_none")]
    pub iowait: f64,
    /// The system load over the last 15 minutes.
    #[serde(rename = "load", skip_serializing_if = "Option::is_none")]
    pub load: f64,
}


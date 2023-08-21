// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsRetry {
    /// Number of times a test needs to be retried before marking a
location as failed. Defaults to 0.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// Time interval between retries (in milliseconds). Defaults to
300ms.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: f64,
}


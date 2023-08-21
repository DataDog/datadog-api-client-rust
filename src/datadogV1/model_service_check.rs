// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCheck {
    /// The check.
    #[serde(rename = "check", skip_serializing_if = "Option::is_none")]
    pub check: String,
    /// The host name correlated with the check.
    #[serde(rename = "host_name", skip_serializing_if = "Option::is_none")]
    pub host_name: String,
    /// Message containing check status.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The status of a service check. Set to `0` for OK, `1` for warning, `2` for critical, and `3` for unknown.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: ServiceCheckStatus,
    /// Tags related to a check.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Time of check.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: i64,
}


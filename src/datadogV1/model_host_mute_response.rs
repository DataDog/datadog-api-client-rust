// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMuteResponse {
    /// Action applied to the hosts.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: String,
    /// POSIX timestamp in seconds when the host is unmuted.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: i64,
    /// The host name.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: String,
    /// Message associated with the mute.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
}


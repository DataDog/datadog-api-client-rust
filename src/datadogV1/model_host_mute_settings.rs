// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMuteSettings {
    /// POSIX timestamp in seconds when the host is unmuted. If omitted, the host remains muted until explicitly unmuted.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: i64,
    /// Message to associate with the muting of this host.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// If true and the host is already muted, replaces existing host mute settings.
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    pub override_: bool,
}


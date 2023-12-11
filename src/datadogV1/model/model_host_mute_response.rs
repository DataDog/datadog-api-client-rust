// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with the list of muted host for your organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostMuteResponse {
    /// Action applied to the hosts.
    #[serde(rename = "action")]
    pub action: Option<String>,
    /// POSIX timestamp in seconds when the host is unmuted.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// The host name.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Message associated with the mute.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl HostMuteResponse {
    pub fn new() -> HostMuteResponse {
        HostMuteResponse {
            action: None,
            end: None,
            hostname: None,
            message: None,
        }
    }
}

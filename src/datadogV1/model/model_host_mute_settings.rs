// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Combination of settings to mute a host.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMuteSettings {
    /// POSIX timestamp in seconds when the host is unmuted. If omitted, the host remains muted until explicitly unmuted.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// Message to associate with the muting of this host.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// If true and the host is already muted, replaces existing host mute settings.
    #[serde(rename = "override")]
    pub override_: Option<bool>,
}

impl HostMuteSettings {
    pub fn new() -> HostMuteSettings {
        HostMuteSettings {
            end: None,
            message: None,
            override_: None,
        }
    }

    pub fn with_end(&mut self, value: i64) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn with_message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn with_override_(&mut self, value: bool) -> &mut Self {
        self.override_ = Some(value);
        self
    }
}
impl Default for HostMuteSettings {
    fn default() -> Self {
        Self::new()
    }
}

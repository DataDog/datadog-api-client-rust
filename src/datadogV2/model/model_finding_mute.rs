// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Information about the mute status of this finding.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindingMute {
    /// Additional information about the reason why this finding is muted or unmuted.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The expiration date of the mute or unmute action (Unix ms).
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    /// Whether this finding is muted or unmuted.
    #[serde(rename = "muted")]
    pub muted: Option<bool>,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason")]
    pub reason: Option<crate::datadogV2::model::FindingMuteReason>,
    /// The start of the mute period.
    #[serde(rename = "start_date")]
    pub start_date: Option<i64>,
    /// The ID of the user who muted or unmuted this finding.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
}

impl FindingMute {
    pub fn new() -> FindingMute {
        FindingMute {
            description: None,
            expiration_date: None,
            muted: None,
            reason: None,
            start_date: None,
            uuid: None,
        }
    }
}
impl Default for FindingMute {
    fn default() -> Self {
        Self::new()
    }
}

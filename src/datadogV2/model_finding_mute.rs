// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindingMute {
    /// Additional information about the reason why this finding is muted or unmuted.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// The expiration date of the mute or unmute action (Unix ms).
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: i64,
    /// Whether this finding is muted or unmuted.
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: bool,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: FindingMuteReason,
    /// The start of the mute period.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: i64,
    /// The ID of the user who muted or unmuted this finding.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: String,
}


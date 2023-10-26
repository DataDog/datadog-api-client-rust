// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingResponseProperties {
    /// Additional information about the reason why this finding is muted or unmuted.
    /// This attribute will not be included in the response if the description is not provided in the request body.
    ///
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The expiration date of the mute or unmute action.
    /// If the expiration date is not provided in the request body, this attribute will not be included in the response and the finding will be muted or unmuted indefinitely.
    ///
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
    /// Whether this finding is muted or unmuted.
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<crate::datadogV2::model::FindingMuteReason>,
}

impl MuteFindingResponseProperties {
    /// Information about the mute status of this finding.
    pub fn new() -> MuteFindingResponseProperties {
        MuteFindingResponseProperties {
            description: None,
            expiration_date: None,
            muted: None,
            reason: None,
        }
    }
}

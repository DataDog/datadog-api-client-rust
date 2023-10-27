// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingRequestProperties {
    /// Additional information about the reason why this finding is muted or unmuted. This field has a maximum limit of 280 characters.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The expiration date of the mute or unmute action (Unix ms). It must be set to a value greater than the current timestamp.
    /// If this field is not provided, the finding will be muted or unmuted indefinitely, which is equivalent to setting the expiration date to 9999999999999.
    ///
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    /// Whether this finding is muted or unmuted.
    #[serde(rename = "muted")]
    pub muted: bool,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason")]
    pub reason: crate::datadogV2::model::FindingMuteReason,
}

impl MuteFindingRequestProperties {
    /// Object containing the new mute properties of the finding.
    pub fn new(muted: bool, reason: crate::datadogV2::model::FindingMuteReason) -> MuteFindingRequestProperties {
        MuteFindingRequestProperties {
            description: None,
            expiration_date: None,
            muted,
            reason,
        }
    }
}

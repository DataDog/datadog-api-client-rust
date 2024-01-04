// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing the change of state for a given state.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalStateUpdateRequest {
    /// Optional comment to explain why a signal is being archived.
    #[serde(rename = "archiveComment")]
    pub archive_comment: Option<String>,
    /// Reason why a signal has been archived.
    #[serde(rename = "archiveReason")]
    pub archive_reason: Option<crate::datadogV1::model::SignalArchiveReason>,
    /// The new triage state of the signal.
    #[serde(rename = "state")]
    pub state: crate::datadogV1::model::SignalTriageState,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SignalStateUpdateRequest {
    pub fn new(state: crate::datadogV1::model::SignalTriageState) -> SignalStateUpdateRequest {
        SignalStateUpdateRequest {
            archive_comment: None,
            archive_reason: None,
            state,
            version: None,
        }
    }
}

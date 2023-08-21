// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalStateUpdateRequest {
    /// Optional comment to explain why a signal is being archived.
    #[serde(rename = "archiveComment", skip_serializing_if = "Option::is_none")]
    pub archive_comment: String,
    /// Reason why a signal has been archived.
    #[serde(rename = "archiveReason", skip_serializing_if = "Option::is_none")]
    pub archive_reason: SignalArchiveReason,
    /// The new triage state of the signal.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: SignalTriageState,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
}


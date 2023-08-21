// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalTriageAttributes {
    /// Optional comment to display on archived signals.
    #[serde(rename = "archive_comment", skip_serializing_if = "Option::is_none")]
    pub archive_comment: String,
    /// Timestamp of the last edit to the comment.
    #[serde(rename = "archive_comment_timestamp", skip_serializing_if = "Option::is_none")]
    pub archive_comment_timestamp: i64,
    /// Object representing a given user entity.
    #[serde(rename = "archive_comment_user")]
    pub archive_comment_user: SecurityMonitoringTriageUser,
    /// Reason a signal is archived.
    #[serde(rename = "archive_reason", skip_serializing_if = "Option::is_none")]
    pub archive_reason: SecurityMonitoringSignalArchiveReason,
    /// Object representing a given user entity.
    #[serde(rename = "assignee")]
    pub assignee: SecurityMonitoringTriageUser,
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids", skip_serializing_if = "Option::is_none")]
    pub incident_ids: Vec<i64>,
    /// The new triage state of the signal.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: SecurityMonitoringSignalState,
    /// Timestamp of the last update to the signal state.
    #[serde(rename = "state_update_timestamp", skip_serializing_if = "Option::is_none")]
    pub state_update_timestamp: i64,
    /// Object representing a given user entity.
    #[serde(rename = "state_update_user")]
    pub state_update_user: SecurityMonitoringTriageUser,
}


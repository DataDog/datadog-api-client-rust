// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing a triage state update operation over a security signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalTriageAttributes {
    /// Optional comment to display on archived signals.
    #[serde(rename = "archive_comment")]
    pub archive_comment: Option<String>,
    /// Timestamp of the last edit to the comment.
    #[serde(rename = "archive_comment_timestamp")]
    pub archive_comment_timestamp: Option<i64>,
    /// Object representing a given user entity.
    #[serde(rename = "archive_comment_user")]
    pub archive_comment_user: Option<crate::datadogV2::model::SecurityMonitoringTriageUser>,
    /// Reason a signal is archived.
    #[serde(rename = "archive_reason")]
    pub archive_reason: Option<crate::datadogV2::model::SecurityMonitoringSignalArchiveReason>,
    /// Object representing a given user entity.
    #[serde(rename = "assignee")]
    pub assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids")]
    pub incident_ids: Vec<i64>,
    /// The new triage state of the signal.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityMonitoringSignalState,
    /// Timestamp of the last update to the signal state.
    #[serde(rename = "state_update_timestamp")]
    pub state_update_timestamp: Option<i64>,
    /// Object representing a given user entity.
    #[serde(rename = "state_update_user")]
    pub state_update_user: Option<crate::datadogV2::model::SecurityMonitoringTriageUser>,
}

impl SecurityMonitoringSignalTriageAttributes {
    pub fn new(
        assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
        incident_ids: Vec<i64>,
        state: crate::datadogV2::model::SecurityMonitoringSignalState,
    ) -> SecurityMonitoringSignalTriageAttributes {
        SecurityMonitoringSignalTriageAttributes {
            archive_comment: None,
            archive_comment_timestamp: None,
            archive_comment_user: None,
            archive_reason: None,
            assignee,
            incident_ids,
            state,
            state_update_timestamp: None,
            state_update_user: None,
        }
    }

    pub fn with_archive_comment(&mut self, value: String) -> &mut Self {
        self.archive_comment = Some(value);
        self
    }

    pub fn with_archive_comment_timestamp(&mut self, value: i64) -> &mut Self {
        self.archive_comment_timestamp = Some(value);
        self
    }

    pub fn with_archive_comment_user(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> &mut Self {
        self.archive_comment_user = Some(value);
        self
    }

    pub fn with_archive_reason(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
    ) -> &mut Self {
        self.archive_reason = Some(value);
        self
    }

    pub fn with_state_update_timestamp(&mut self, value: i64) -> &mut Self {
        self.state_update_timestamp = Some(value);
        self
    }

    pub fn with_state_update_user(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> &mut Self {
        self.state_update_user = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing the change of state of a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalStateUpdateAttributes {
    /// Optional comment to display on archived signals.
    #[serde(rename = "archive_comment")]
    pub archive_comment: Option<String>,
    /// Reason a signal is archived.
    #[serde(rename = "archive_reason")]
    pub archive_reason: Option<crate::datadogV2::model::SecurityMonitoringSignalArchiveReason>,
    /// The new triage state of the signal.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityMonitoringSignalState,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalStateUpdateAttributes {
    pub fn new(
        state: crate::datadogV2::model::SecurityMonitoringSignalState,
    ) -> SecurityMonitoringSignalStateUpdateAttributes {
        SecurityMonitoringSignalStateUpdateAttributes {
            archive_comment: None,
            archive_reason: None,
            state,
            version: None,
        }
    }

    pub fn archive_comment(&mut self, value: String) -> &mut Self {
        self.archive_comment = Some(value);
        self
    }

    pub fn archive_reason(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
    ) -> &mut Self {
        self.archive_reason = Some(value);
        self
    }

    pub fn version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}

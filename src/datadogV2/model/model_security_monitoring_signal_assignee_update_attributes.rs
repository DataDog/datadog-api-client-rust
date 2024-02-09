// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing the new assignee of a security signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalAssigneeUpdateAttributes {
    /// Object representing a given user entity.
    #[serde(rename = "assignee")]
    pub assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalAssigneeUpdateAttributes {
    pub fn new(
        assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> SecurityMonitoringSignalAssigneeUpdateAttributes {
        SecurityMonitoringSignalAssigneeUpdateAttributes {
            assignee,
            version: None,
        }
    }

    pub fn version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}

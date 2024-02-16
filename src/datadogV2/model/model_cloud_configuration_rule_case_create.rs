// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of signals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleCaseCreate {
    /// Notification targets for each rule case.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
}

impl CloudConfigurationRuleCaseCreate {
    pub fn new(
        status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> CloudConfigurationRuleCaseCreate {
        CloudConfigurationRuleCaseCreate {
            notifications: None,
            status,
        }
    }

    pub fn notifications(&mut self, value: Vec<String>) -> &mut Self {
        self.notifications = Some(value);
        self
    }
}

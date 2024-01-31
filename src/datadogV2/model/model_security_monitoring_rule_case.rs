// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Case when signal is generated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleCase {
    /// A rule case contains logical operations (`>`,`>=`, `&&`, `||`) to determine if a signal should be generated
    /// based on the event counts in the previously defined queries.
    #[serde(rename = "condition")]
    pub condition: Option<String>,
    /// Name of the case.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Notification targets for each rule case.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
}

impl SecurityMonitoringRuleCase {
    pub fn new() -> SecurityMonitoringRuleCase {
        SecurityMonitoringRuleCase {
            condition: None,
            name: None,
            notifications: None,
            status: None,
        }
    }

    pub fn with_condition(&mut self, value: String) -> &mut Self {
        self.condition = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_notifications(&mut self, value: Vec<String>) -> &mut Self {
        self.notifications = Some(value);
        self
    }

    pub fn with_status(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> &mut Self {
        self.status = Some(value);
        self
    }
}
impl Default for SecurityMonitoringRuleCase {
    fn default() -> Self {
        Self::new()
    }
}

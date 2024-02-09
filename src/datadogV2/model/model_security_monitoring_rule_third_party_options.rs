// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options on third party rules.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleThirdPartyOptions {
    /// Notification targets for the logs that do not correspond to any of the cases.
    #[serde(rename = "defaultNotifications")]
    pub default_notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "defaultStatus")]
    pub default_status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
    /// Queries to be combined with third party case queries. Each of them can have different group by fields, to aggregate differently based on the type of alert.
    #[serde(rename = "rootQueries")]
    pub root_queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRootQuery>>,
    /// A template for the signal title; if omitted, the title is generated based on the case name.
    #[serde(rename = "signalTitleTemplate")]
    pub signal_title_template: Option<String>,
}

impl SecurityMonitoringRuleThirdPartyOptions {
    pub fn new() -> SecurityMonitoringRuleThirdPartyOptions {
        SecurityMonitoringRuleThirdPartyOptions {
            default_notifications: None,
            default_status: None,
            root_queries: None,
            signal_title_template: None,
        }
    }

    pub fn default_notifications(&mut self, value: Vec<String>) -> &mut Self {
        self.default_notifications = Some(value);
        self
    }

    pub fn default_status(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> &mut Self {
        self.default_status = Some(value);
        self
    }

    pub fn root_queries(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRootQuery>,
    ) -> &mut Self {
        self.root_queries = Some(value);
        self
    }

    pub fn signal_title_template(&mut self, value: String) -> &mut Self {
        self.signal_title_template = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleThirdPartyOptions {
    fn default() -> Self {
        Self::new()
    }
}

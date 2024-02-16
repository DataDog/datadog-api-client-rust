// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The rule's suppression filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringFilter {
    /// The type of filtering action.
    #[serde(rename = "action")]
    pub action: Option<crate::datadogV2::model::SecurityMonitoringFilterAction>,
    /// Query for selecting logs to apply the filtering action.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SecurityMonitoringFilter {
    pub fn new() -> SecurityMonitoringFilter {
        SecurityMonitoringFilter {
            action: None,
            query: None,
        }
    }

    pub fn action(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringFilterAction,
    ) -> &mut Self {
        self.action = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}

impl Default for SecurityMonitoringFilter {
    fn default() -> Self {
        Self::new()
    }
}

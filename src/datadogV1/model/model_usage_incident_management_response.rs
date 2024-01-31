// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the incident management usage for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageIncidentManagementResponse {
    /// Get hourly usage for incident management.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageIncidentManagementHour>>,
}

impl UsageIncidentManagementResponse {
    pub fn new() -> UsageIncidentManagementResponse {
        UsageIncidentManagementResponse { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageIncidentManagementHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for UsageIncidentManagementResponse {
    fn default() -> Self {
        Self::new()
    }
}

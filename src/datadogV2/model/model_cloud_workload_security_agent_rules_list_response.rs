// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object that includes a list of Agent rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRulesListResponse {
    /// A list of Agent rules objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleData>>,
}

impl CloudWorkloadSecurityAgentRulesListResponse {
    pub fn new() -> CloudWorkloadSecurityAgentRulesListResponse {
        CloudWorkloadSecurityAgentRulesListResponse { data: None }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for CloudWorkloadSecurityAgentRulesListResponse {
    fn default() -> Self {
        Self::new()
    }
}

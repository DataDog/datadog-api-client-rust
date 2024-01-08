// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single Agent rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleData {
    /// A Cloud Workload Security Agent rule returned by the API.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAttributes>>,
    /// The ID of the Agent rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the resource. The value should always be `agent_rule`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleType>,
}

impl CloudWorkloadSecurityAgentRuleData {
    pub fn new() -> CloudWorkloadSecurityAgentRuleData {
        CloudWorkloadSecurityAgentRuleData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for CloudWorkloadSecurityAgentRuleData {
    fn default() -> Self {
        Self::new()
    }
}

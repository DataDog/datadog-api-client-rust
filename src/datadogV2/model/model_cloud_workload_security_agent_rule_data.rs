// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleData {
    /// A Cloud Workload Security Agent rule returned by the API.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAttributes>>,
    /// The ID of the Agent rule.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the resource. The value should always be `agent_rule`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleType>,
}

impl CloudWorkloadSecurityAgentRuleData {
    /// Object for a single Agent rule.
    pub fn new() -> CloudWorkloadSecurityAgentRuleData {
        CloudWorkloadSecurityAgentRuleData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}

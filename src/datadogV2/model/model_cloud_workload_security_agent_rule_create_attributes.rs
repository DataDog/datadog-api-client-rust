// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleCreateAttributes {
    /// The description of the Agent rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression")]
    pub expression: String,
    /// The name of the Agent rule.
    #[serde(rename = "name")]
    pub name: String,
}

impl CloudWorkloadSecurityAgentRuleCreateAttributes {
    /// Create a new Cloud Workload Security Agent rule.
    pub fn new(expression: String, name: String) -> CloudWorkloadSecurityAgentRuleCreateAttributes {
        CloudWorkloadSecurityAgentRuleCreateAttributes {
            description: None,
            enabled: None,
            expression: expression,
            name: name,
        }
    }
}

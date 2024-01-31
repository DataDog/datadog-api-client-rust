// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update an existing Cloud Workload Security Agent rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateAttributes {
    /// The description of the Agent rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
}

impl CloudWorkloadSecurityAgentRuleUpdateAttributes {
    pub fn new() -> CloudWorkloadSecurityAgentRuleUpdateAttributes {
        CloudWorkloadSecurityAgentRuleUpdateAttributes {
            description: None,
            enabled: None,
            expression: None,
        }
    }

    pub fn with_description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn with_enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = Some(value);
        self
    }

    pub fn with_expression(&mut self, value: String) -> &mut Self {
        self.expression = Some(value);
        self
    }
}
impl Default for CloudWorkloadSecurityAgentRuleUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

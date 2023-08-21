// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateAttributes {
    /// The description of the Agent rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: bool,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: String,
}


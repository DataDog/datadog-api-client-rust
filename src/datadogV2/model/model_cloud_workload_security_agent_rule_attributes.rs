// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleAttributes {
    /// The version of the agent.
    #[serde(rename = "agentConstraint", skip_serializing_if = "Option::is_none")]
    pub agent_constraint: Option<String>,
    /// The category of the Agent rule.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The ID of the user who created the rule.
    #[serde(rename = "creationAuthorUuId", skip_serializing_if = "Option::is_none")]
    pub creation_author_uu_id: Option<String>,
    /// When the Agent rule was created, timestamp in milliseconds.
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<i64>,
    /// The attributes of the user who created the Agent rule.
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreatorAttributes>>,
    /// Whether the rule is included by default.
    #[serde(rename = "defaultRule", skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<bool>,
    /// The description of the Agent rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// The name of the Agent rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the user who updated the rule.
    #[serde(rename = "updateAuthorUuId", skip_serializing_if = "Option::is_none")]
    pub update_author_uu_id: Option<String>,
    /// Timestamp in milliseconds when the Agent rule was last updated.
    #[serde(rename = "updateDate", skip_serializing_if = "Option::is_none")]
    pub update_date: Option<i64>,
    /// When the Agent rule was last updated, timestamp in milliseconds.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// The attributes of the user who last updated the Agent rule.
    #[serde(rename = "updater", skip_serializing_if = "Option::is_none")]
    pub updater: Option<Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdaterAttributes>>,
    /// The version of the Agent rule.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl CloudWorkloadSecurityAgentRuleAttributes {
    /// A Cloud Workload Security Agent rule returned by the API.
    pub fn new() -> CloudWorkloadSecurityAgentRuleAttributes {
        CloudWorkloadSecurityAgentRuleAttributes {
            agent_constraint: None,
            category: None,
            creation_author_uu_id: None,
            creation_date: None,
            creator: None,
            default_rule: None,
            description: None,
            enabled: None,
            expression: None,
            name: None,
            update_author_uu_id: None,
            update_date: None,
            updated_at: None,
            updater: None,
            version: None,
        }
    }
}

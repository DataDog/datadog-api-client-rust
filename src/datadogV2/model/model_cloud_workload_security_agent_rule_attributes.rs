// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A Cloud Workload Security Agent rule returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleAttributes {
    /// The version of the agent.
    #[serde(rename = "agentConstraint")]
    pub agent_constraint: Option<String>,
    /// The category of the Agent rule.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// The ID of the user who created the rule.
    #[serde(rename = "creationAuthorUuId")]
    pub creation_author_uu_id: Option<String>,
    /// When the Agent rule was created, timestamp in milliseconds.
    #[serde(rename = "creationDate")]
    pub creation_date: Option<i64>,
    /// The attributes of the user who created the Agent rule.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreatorAttributes>,
    /// Whether the rule is included by default.
    #[serde(rename = "defaultRule")]
    pub default_rule: Option<bool>,
    /// The description of the Agent rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// The platforms the Agent rule is supported on.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// The name of the Agent rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The ID of the user who updated the rule.
    #[serde(rename = "updateAuthorUuId")]
    pub update_author_uu_id: Option<String>,
    /// Timestamp in milliseconds when the Agent rule was last updated.
    #[serde(rename = "updateDate")]
    pub update_date: Option<i64>,
    /// When the Agent rule was last updated, timestamp in milliseconds.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    /// The attributes of the user who last updated the Agent rule.
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdaterAttributes>,
    /// The version of the Agent rule.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl CloudWorkloadSecurityAgentRuleAttributes {
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
            filters: None,
            name: None,
            update_author_uu_id: None,
            update_date: None,
            updated_at: None,
            updater: None,
            version: None,
        }
    }

    pub fn agent_constraint(mut self, value: String) -> Self {
        self.agent_constraint = Some(value);
        self
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn creation_author_uu_id(mut self, value: String) -> Self {
        self.creation_author_uu_id = Some(value);
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn creator(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreatorAttributes,
    ) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn default_rule(mut self, value: bool) -> Self {
        self.default_rule = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn expression(mut self, value: String) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<String>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn update_author_uu_id(mut self, value: String) -> Self {
        self.update_author_uu_id = Some(value);
        self
    }

    pub fn update_date(mut self, value: i64) -> Self {
        self.update_date = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updater(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdaterAttributes,
    ) -> Self {
        self.updater = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for CloudWorkloadSecurityAgentRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

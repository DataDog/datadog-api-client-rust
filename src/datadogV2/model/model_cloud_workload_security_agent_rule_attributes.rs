// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Cloud Workload Security Agent rule returned by the API
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleAttributes {
    /// The array of actions the rule can perform if triggered
    #[serde(
        rename = "actions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub actions: Option<Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAction>>>,
    /// The version of the Agent
    #[serde(rename = "agentConstraint")]
    pub agent_constraint: Option<String>,
    /// The blocking policies that the rule belongs to
    #[serde(rename = "blocking")]
    pub blocking: Option<Vec<String>>,
    /// The category of the Agent rule
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// The ID of the user who created the rule
    #[serde(rename = "creationAuthorUuId")]
    pub creation_author_uu_id: Option<String>,
    /// When the Agent rule was created, timestamp in milliseconds
    #[serde(rename = "creationDate")]
    pub creation_date: Option<i64>,
    /// The attributes of the user who created the Agent rule
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreatorAttributes>,
    /// Whether the rule is included by default
    #[serde(rename = "defaultRule")]
    pub default_rule: Option<bool>,
    /// The description of the Agent rule
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The disabled policies that the rule belongs to
    #[serde(rename = "disabled")]
    pub disabled: Option<Vec<String>>,
    /// Whether the Agent rule is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// The platforms the Agent rule is supported on
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// The monitoring policies that the rule belongs to
    #[serde(rename = "monitoring")]
    pub monitoring: Option<Vec<String>>,
    /// The name of the Agent rule
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The list of product tags associated with the rule
    #[serde(rename = "product_tags")]
    pub product_tags: Option<Vec<String>>,
    /// The ID of the user who updated the rule
    #[serde(rename = "updateAuthorUuId")]
    pub update_author_uu_id: Option<String>,
    /// Timestamp in milliseconds when the Agent rule was last updated
    #[serde(rename = "updateDate")]
    pub update_date: Option<i64>,
    /// When the Agent rule was last updated, timestamp in milliseconds
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    /// The attributes of the user who last updated the Agent rule
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdaterAttributes>,
    /// The version of the Agent rule
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleAttributes {
    pub fn new() -> CloudWorkloadSecurityAgentRuleAttributes {
        CloudWorkloadSecurityAgentRuleAttributes {
            actions: None,
            agent_constraint: None,
            blocking: None,
            category: None,
            creation_author_uu_id: None,
            creation_date: None,
            creator: None,
            default_rule: None,
            description: None,
            disabled: None,
            enabled: None,
            expression: None,
            filters: None,
            monitoring: None,
            name: None,
            product_tags: None,
            update_author_uu_id: None,
            update_date: None,
            updated_at: None,
            updater: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actions(
        mut self,
        value: Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAction>>,
    ) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn agent_constraint(mut self, value: String) -> Self {
        self.agent_constraint = Some(value);
        self
    }

    pub fn blocking(mut self, value: Vec<String>) -> Self {
        self.blocking = Some(value);
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

    pub fn disabled(mut self, value: Vec<String>) -> Self {
        self.disabled = Some(value);
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

    pub fn monitoring(mut self, value: Vec<String>) -> Self {
        self.monitoring = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn product_tags(mut self, value: Vec<String>) -> Self {
        self.product_tags = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for CloudWorkloadSecurityAgentRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions: Option<
                    Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAction>>,
                > = None;
                let mut agent_constraint: Option<String> = None;
                let mut blocking: Option<Vec<String>> = None;
                let mut category: Option<String> = None;
                let mut creation_author_uu_id: Option<String> = None;
                let mut creation_date: Option<i64> = None;
                let mut creator: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreatorAttributes,
                > = None;
                let mut default_rule: Option<bool> = None;
                let mut description: Option<String> = None;
                let mut disabled: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut expression: Option<String> = None;
                let mut filters: Option<Vec<String>> = None;
                let mut monitoring: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut product_tags: Option<Vec<String>> = None;
                let mut update_author_uu_id: Option<String> = None;
                let mut update_date: Option<i64> = None;
                let mut updated_at: Option<i64> = None;
                let mut updater: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdaterAttributes,
                > = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions" => {
                            actions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agentConstraint" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_constraint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "blocking" => {
                            if v.is_null() {
                                continue;
                            }
                            blocking = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creationAuthorUuId" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_author_uu_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creationDate" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultRule" => {
                            if v.is_null() {
                                continue;
                            }
                            default_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            if v.is_null() {
                                continue;
                            }
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitoring" => {
                            if v.is_null() {
                                continue;
                            }
                            monitoring = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            product_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updateAuthorUuId" => {
                            if v.is_null() {
                                continue;
                            }
                            update_author_uu_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updateDate" => {
                            if v.is_null() {
                                continue;
                            }
                            update_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater" => {
                            if v.is_null() {
                                continue;
                            }
                            updater = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleAttributes {
                    actions,
                    agent_constraint,
                    blocking,
                    category,
                    creation_author_uu_id,
                    creation_date,
                    creator,
                    default_rule,
                    description,
                    disabled,
                    enabled,
                    expression,
                    filters,
                    monitoring,
                    name,
                    product_tags,
                    update_author_uu_id,
                    update_date,
                    updated_at,
                    updater,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create a new Cloud Workload Security Agent rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleCreateAttributes {
    /// The array of actions the rule can perform if triggered
    #[serde(
        rename = "actions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub actions: Option<Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleAction>>>,
    /// The blocking policies that the rule belongs to
    #[serde(rename = "blocking")]
    pub blocking: Option<Vec<String>>,
    /// The description of the Agent rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The disabled policies that the rule belongs to
    #[serde(rename = "disabled")]
    pub disabled: Option<Vec<String>>,
    /// Whether the Agent rule is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression")]
    pub expression: String,
    /// The platforms the Agent rule is supported on
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// The monitoring policies that the rule belongs to
    #[serde(rename = "monitoring")]
    pub monitoring: Option<Vec<String>>,
    /// The name of the Agent rule.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the policy where the Agent rule is saved
    #[serde(rename = "policy_id")]
    pub policy_id: Option<String>,
    /// The list of product tags associated with the rule
    #[serde(rename = "product_tags")]
    pub product_tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleCreateAttributes {
    pub fn new(expression: String, name: String) -> CloudWorkloadSecurityAgentRuleCreateAttributes {
        CloudWorkloadSecurityAgentRuleCreateAttributes {
            actions: None,
            blocking: None,
            description: None,
            disabled: None,
            enabled: None,
            expression,
            filters: None,
            monitoring: None,
            name,
            policy_id: None,
            product_tags: None,
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

    pub fn blocking(mut self, value: Vec<String>) -> Self {
        self.blocking = Some(value);
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

    pub fn filters(mut self, value: Vec<String>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn monitoring(mut self, value: Vec<String>) -> Self {
        self.monitoring = Some(value);
        self
    }

    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }

    pub fn product_tags(mut self, value: Vec<String>) -> Self {
        self.product_tags = Some(value);
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

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleCreateAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentRuleCreateAttributes;

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
                let mut blocking: Option<Vec<String>> = None;
                let mut description: Option<String> = None;
                let mut disabled: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut expression: Option<String> = None;
                let mut filters: Option<Vec<String>> = None;
                let mut monitoring: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut policy_id: Option<String> = None;
                let mut product_tags: Option<Vec<String>> = None;
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
                        "blocking" => {
                            if v.is_null() {
                                continue;
                            }
                            blocking = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_id" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            product_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CloudWorkloadSecurityAgentRuleCreateAttributes {
                    actions,
                    blocking,
                    description,
                    disabled,
                    enabled,
                    expression,
                    filters,
                    monitoring,
                    name,
                    policy_id,
                    product_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleCreateAttributesVisitor)
    }
}

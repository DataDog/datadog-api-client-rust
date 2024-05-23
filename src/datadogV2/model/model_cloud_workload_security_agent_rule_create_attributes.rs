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
    /// The description of the Agent rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the Agent rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The SECL expression of the Agent rule.
    #[serde(rename = "expression")]
    pub expression: String,
    /// The platforms the Agent rule is supported on.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// The name of the Agent rule.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleCreateAttributes {
    pub fn new(expression: String, name: String) -> CloudWorkloadSecurityAgentRuleCreateAttributes {
        CloudWorkloadSecurityAgentRuleCreateAttributes {
            description: None,
            enabled: None,
            expression,
            filters: None,
            name,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut expression: Option<String> = None;
                let mut filters: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CloudWorkloadSecurityAgentRuleCreateAttributes {
                    description,
                    enabled,
                    expression,
                    filters,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleCreateAttributesVisitor)
    }
}

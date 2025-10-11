// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The set action applied on the scope matching the rule
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleActionSet {
    /// Whether the value should be appended to the field.
    #[serde(rename = "append")]
    pub append: Option<bool>,
    /// The default value of the set action
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    /// The expression of the set action.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// The field of the set action
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// Whether the value should be inherited.
    #[serde(rename = "inherited")]
    pub inherited: Option<bool>,
    /// The name of the set action
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The scope of the set action.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The size of the set action.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// The time to live of the set action.
    #[serde(rename = "ttl")]
    pub ttl: Option<i64>,
    /// The value of the set action
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleActionSet {
    pub fn new() -> CloudWorkloadSecurityAgentRuleActionSet {
        CloudWorkloadSecurityAgentRuleActionSet {
            append: None,
            default_value: None,
            expression: None,
            field: None,
            inherited: None,
            name: None,
            scope: None,
            size: None,
            ttl: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn append(mut self, value: bool) -> Self {
        self.append = Some(value);
        self
    }

    pub fn default_value(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn expression(mut self, value: String) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn inherited(mut self, value: bool) -> Self {
        self.inherited = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
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

impl Default for CloudWorkloadSecurityAgentRuleActionSet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleActionSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleActionSetVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleActionSetVisitor {
            type Value = CloudWorkloadSecurityAgentRuleActionSet;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut append: Option<bool> = None;
                let mut default_value: Option<String> = None;
                let mut expression: Option<String> = None;
                let mut field: Option<String> = None;
                let mut inherited: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut ttl: Option<i64> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "append" => {
                            if v.is_null() {
                                continue;
                            }
                            append = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_value" => {
                            if v.is_null() {
                                continue;
                            }
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            if v.is_null() {
                                continue;
                            }
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inherited" => {
                            if v.is_null() {
                                continue;
                            }
                            inherited = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ttl" => {
                            if v.is_null() {
                                continue;
                            }
                            ttl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleActionSet {
                    append,
                    default_value,
                    expression,
                    field,
                    inherited,
                    name,
                    scope,
                    size,
                    ttl,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleActionSetVisitor)
    }
}

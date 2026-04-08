// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a rule. Server-managed fields (created_at, modified_at, custom) are excluded.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleAttributesRequest {
    /// Explanation of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// If enabled, the rule is calculated as part of the score.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The maturity level of the rule (1, 2, or 3).
    #[serde(rename = "level")]
    pub level: Option<i32>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Owner of the rule.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    /// A query to filter which entities this rule applies to.
    #[serde(rename = "scope_query")]
    pub scope_query: Option<String>,
    /// The scorecard name to which this rule must belong.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleAttributesRequest {
    pub fn new() -> RuleAttributesRequest {
        RuleAttributesRequest {
            description: None,
            enabled: None,
            level: None,
            name: None,
            owner: None,
            scope_query: None,
            scorecard_name: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn level(mut self, value: i32) -> Self {
        self.level = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn owner(mut self, value: String) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn scope_query(mut self, value: String) -> Self {
        self.scope_query = Some(value);
        self
    }

    pub fn scorecard_name(mut self, value: String) -> Self {
        self.scorecard_name = Some(value);
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

impl Default for RuleAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RuleAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleAttributesRequestVisitor;
        impl<'a> Visitor<'a> for RuleAttributesRequestVisitor {
            type Value = RuleAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut level: Option<i32> = None;
                let mut name: Option<String> = None;
                let mut owner: Option<String> = None;
                let mut scope_query: Option<String> = None;
                let mut scorecard_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner" => {
                            if v.is_null() {
                                continue;
                            }
                            owner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_query" => {
                            if v.is_null() {
                                continue;
                            }
                            scope_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scorecard_name" => {
                            if v.is_null() {
                                continue;
                            }
                            scorecard_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RuleAttributesRequest {
                    description,
                    enabled,
                    level,
                    name,
                    owner,
                    scope_query,
                    scorecard_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleAttributesRequestVisitor)
    }
}

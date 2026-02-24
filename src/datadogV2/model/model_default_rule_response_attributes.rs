// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Default rule attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DefaultRuleResponseAttributes {
    /// The description of the default rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The maturity level of the rule.
    #[serde(rename = "level")]
    pub level: Option<i64>,
    /// The name of the default rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Required scope for the rule.
    #[serde(rename = "scope_required")]
    pub scope_required: Option<String>,
    /// The description of the scorecard.
    #[serde(rename = "scorecard_description")]
    pub scorecard_description: Option<String>,
    /// The scorecard this rule belongs to.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DefaultRuleResponseAttributes {
    pub fn new(name: String, scorecard_name: String) -> DefaultRuleResponseAttributes {
        DefaultRuleResponseAttributes {
            description: None,
            level: None,
            name,
            scope_required: None,
            scorecard_description: None,
            scorecard_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    pub fn scope_required(mut self, value: String) -> Self {
        self.scope_required = Some(value);
        self
    }

    pub fn scorecard_description(mut self, value: String) -> Self {
        self.scorecard_description = Some(value);
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

impl<'de> Deserialize<'de> for DefaultRuleResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DefaultRuleResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DefaultRuleResponseAttributesVisitor {
            type Value = DefaultRuleResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut level: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut scope_required: Option<String> = None;
                let mut scorecard_description: Option<String> = None;
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
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_required" => {
                            if v.is_null() {
                                continue;
                            }
                            scope_required =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scorecard_description" => {
                            if v.is_null() {
                                continue;
                            }
                            scorecard_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scorecard_name" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let scorecard_name =
                    scorecard_name.ok_or_else(|| M::Error::missing_field("scorecard_name"))?;

                let content = DefaultRuleResponseAttributes {
                    description,
                    level,
                    name,
                    scope_required,
                    scorecard_description,
                    scorecard_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DefaultRuleResponseAttributesVisitor)
    }
}

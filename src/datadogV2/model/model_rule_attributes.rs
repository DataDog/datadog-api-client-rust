// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleAttributes {
    /// The scorecard name to which this rule must belong.
    #[deprecated]
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Creation time of the rule outcome.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Defines if the rule is a custom rule.
    #[serde(rename = "custom")]
    pub custom: Option<bool>,
    /// Explanation of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// If enabled, the rule is calculated as part of the score.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Time of the last rule outcome modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Owner of the rule.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    /// The scorecard name to which this rule must belong.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleAttributes {
    pub fn new() -> RuleAttributes {
        #[allow(deprecated)]
        RuleAttributes {
            category: None,
            created_at: None,
            custom: None,
            description: None,
            enabled: None,
            modified_at: None,
            name: None,
            owner: None,
            scorecard_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom(mut self, value: bool) -> Self {
        self.custom = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn owner(mut self, value: String) -> Self {
        self.owner = Some(value);
        self
    }

    #[allow(deprecated)]
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

impl Default for RuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleAttributesVisitor;
        impl<'a> Visitor<'a> for RuleAttributesVisitor {
            type Value = RuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut custom: Option<bool> = None;
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut owner: Option<String> = None;
                let mut scorecard_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                #[allow(deprecated)]
                let content = RuleAttributes {
                    category,
                    created_at,
                    custom,
                    description,
                    enabled,
                    modified_at,
                    name,
                    owner,
                    scorecard_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleAttributesVisitor)
    }
}

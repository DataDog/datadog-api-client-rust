// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UpdateRulesetRequestDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRulesetRequestDataAttributes {
    /// The `attributes` `enabled`.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The `attributes` `last_version`.
    #[serde(rename = "last_version")]
    pub last_version: Option<i64>,
    /// The `attributes` `rules`.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItems>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateRulesetRequestDataAttributes {
    pub fn new(
        enabled: bool,
        rules: Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItems>,
    ) -> UpdateRulesetRequestDataAttributes {
        UpdateRulesetRequestDataAttributes {
            enabled,
            last_version: None,
            rules,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_version(mut self, value: i64) -> Self {
        self.last_version = Some(value);
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

impl<'de> Deserialize<'de> for UpdateRulesetRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateRulesetRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateRulesetRequestDataAttributesVisitor {
            type Value = UpdateRulesetRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut last_version: Option<i64> = None;
                let mut rules: Option<
                    Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_version" => {
                            if v.is_null() {
                                continue;
                            }
                            last_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;

                let content = UpdateRulesetRequestDataAttributes {
                    enabled,
                    last_version,
                    rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateRulesetRequestDataAttributesVisitor)
    }
}

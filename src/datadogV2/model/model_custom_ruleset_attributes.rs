// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRulesetAttributes {
    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Creator identifier
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Base64-encoded full description
    #[serde(rename = "description")]
    pub description: String,
    /// Ruleset name
    #[serde(rename = "name")]
    pub name: String,
    /// Rules in the ruleset
    #[serialize_always]
    #[serde(rename = "rules")]
    pub rules: Option<Vec<crate::datadogV2::model::CustomRule>>,
    /// Base64-encoded short description
    #[serde(rename = "short_description")]
    pub short_description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRulesetAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        description: String,
        name: String,
        rules: Option<Vec<crate::datadogV2::model::CustomRule>>,
        short_description: String,
    ) -> CustomRulesetAttributes {
        CustomRulesetAttributes {
            created_at,
            created_by,
            description,
            name,
            rules,
            short_description,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CustomRulesetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomRulesetAttributesVisitor;
        impl<'a> Visitor<'a> for CustomRulesetAttributesVisitor {
            type Value = CustomRulesetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut rules: Option<Option<Vec<crate::datadogV2::model::CustomRule>>> = None;
                let mut short_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;

                let content = CustomRulesetAttributes {
                    created_at,
                    created_by,
                    description,
                    name,
                    rules,
                    short_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRulesetAttributesVisitor)
    }
}

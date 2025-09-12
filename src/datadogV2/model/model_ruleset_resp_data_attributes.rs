// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `RulesetRespDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RulesetRespDataAttributes {
    /// The definition of `RulesetRespDataAttributesCreated` object.
    #[serde(rename = "created")]
    pub created: crate::datadogV2::model::RulesetRespDataAttributesCreated,
    /// The `attributes` `enabled`.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The `attributes` `last_modified_user_uuid`.
    #[serde(rename = "last_modified_user_uuid")]
    pub last_modified_user_uuid: String,
    /// The definition of `RulesetRespDataAttributesModified` object.
    #[serde(rename = "modified")]
    pub modified: crate::datadogV2::model::RulesetRespDataAttributesModified,
    /// The `attributes` `name`.
    #[serde(rename = "name")]
    pub name: String,
    /// The `attributes` `position`.
    #[serde(rename = "position")]
    pub position: i32,
    /// The `attributes` `rules`.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::RulesetRespDataAttributesRulesItems>,
    /// The `attributes` `version`.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RulesetRespDataAttributes {
    pub fn new(
        created: crate::datadogV2::model::RulesetRespDataAttributesCreated,
        enabled: bool,
        last_modified_user_uuid: String,
        modified: crate::datadogV2::model::RulesetRespDataAttributesModified,
        name: String,
        position: i32,
        rules: Vec<crate::datadogV2::model::RulesetRespDataAttributesRulesItems>,
        version: i64,
    ) -> RulesetRespDataAttributes {
        RulesetRespDataAttributes {
            created,
            enabled,
            last_modified_user_uuid,
            modified,
            name,
            position,
            rules,
            version,
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

impl<'de> Deserialize<'de> for RulesetRespDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RulesetRespDataAttributesVisitor;
        impl<'a> Visitor<'a> for RulesetRespDataAttributesVisitor {
            type Value = RulesetRespDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<crate::datadogV2::model::RulesetRespDataAttributesCreated> =
                    None;
                let mut enabled: Option<bool> = None;
                let mut last_modified_user_uuid: Option<String> = None;
                let mut modified: Option<
                    crate::datadogV2::model::RulesetRespDataAttributesModified,
                > = None;
                let mut name: Option<String> = None;
                let mut position: Option<i32> = None;
                let mut rules: Option<
                    Vec<crate::datadogV2::model::RulesetRespDataAttributesRulesItems>,
                > = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_user_uuid" => {
                            last_modified_user_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "position" => {
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let last_modified_user_uuid = last_modified_user_uuid
                    .ok_or_else(|| M::Error::missing_field("last_modified_user_uuid"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let position = position.ok_or_else(|| M::Error::missing_field("position"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = RulesetRespDataAttributes {
                    created,
                    enabled,
                    last_modified_user_uuid,
                    modified,
                    name,
                    position,
                    rules,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RulesetRespDataAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a single End User Device Monitoring issue definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueDefinitionDataAttributes {
    /// Category of the issue (for example, `battery`, `network`, or `performance`).
    #[serde(rename = "category")]
    pub category: String,
    /// Human-readable label describing the issue, suitable for display in the Datadog UI.
    #[serde(rename = "label")]
    pub label: String,
    /// Severity level of the issue (for example, `warning` or `critical`).
    #[serde(rename = "level")]
    pub level: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueDefinitionDataAttributes {
    pub fn new(category: String, label: String, level: String) -> IssueDefinitionDataAttributes {
        IssueDefinitionDataAttributes {
            category,
            label,
            level,
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

impl<'de> Deserialize<'de> for IssueDefinitionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueDefinitionDataAttributesVisitor;
        impl<'a> Visitor<'a> for IssueDefinitionDataAttributesVisitor {
            type Value = IssueDefinitionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut label: Option<String> = None;
                let mut level: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "level" => {
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let level = level.ok_or_else(|| M::Error::missing_field("level"))?;

                let content = IssueDefinitionDataAttributes {
                    category,
                    label,
                    level,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueDefinitionDataAttributesVisitor)
    }
}

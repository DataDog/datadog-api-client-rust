// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a widget resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetAttributes {
    /// ISO 8601 timestamp of when the widget was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The definition of a widget, including its type and configuration.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::WidgetDefinition,
    /// Will be implemented soon. Currently always returns false.
    #[serde(rename = "is_favorited")]
    pub is_favorited: bool,
    /// ISO 8601 timestamp of when the widget was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    /// User-defined tags for organizing widgets.
    #[serialize_always]
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetAttributes {
    pub fn new(
        created_at: String,
        definition: crate::datadogV2::model::WidgetDefinition,
        is_favorited: bool,
        modified_at: String,
        tags: Option<Vec<String>>,
    ) -> WidgetAttributes {
        WidgetAttributes {
            created_at,
            definition,
            is_favorited,
            modified_at,
            tags,
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

impl<'de> Deserialize<'de> for WidgetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetAttributesVisitor;
        impl<'a> Visitor<'a> for WidgetAttributesVisitor {
            type Value = WidgetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut definition: Option<crate::datadogV2::model::WidgetDefinition> = None;
                let mut is_favorited: Option<bool> = None;
                let mut modified_at: Option<String> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
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
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorited" => {
                            is_favorited =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let is_favorited =
                    is_favorited.ok_or_else(|| M::Error::missing_field("is_favorited"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = WidgetAttributes {
                    created_at,
                    definition,
                    is_favorited,
                    modified_at,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetAttributesVisitor)
    }
}

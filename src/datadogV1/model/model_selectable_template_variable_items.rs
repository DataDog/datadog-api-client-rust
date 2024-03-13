// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the template variable's name, associated tag/attribute, default value and selectable values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SelectableTemplateVariableItems {
    /// The default value of the template variable.
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    /// Name of the template variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The tag/attribute key associated with the template variable.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// List of visible tag values on the shared dashboard.
    #[serde(
        rename = "visible_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub visible_tags: Option<Option<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SelectableTemplateVariableItems {
    pub fn new() -> SelectableTemplateVariableItems {
        SelectableTemplateVariableItems {
            default_value: None,
            name: None,
            prefix: None,
            visible_tags: None,
            _unparsed: false,
        }
    }

    pub fn default_value(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn prefix(mut self, value: String) -> Self {
        self.prefix = Some(value);
        self
    }

    pub fn visible_tags(mut self, value: Option<Vec<String>>) -> Self {
        self.visible_tags = Some(value);
        self
    }
}

impl Default for SelectableTemplateVariableItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SelectableTemplateVariableItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SelectableTemplateVariableItemsVisitor;
        impl<'a> Visitor<'a> for SelectableTemplateVariableItemsVisitor {
            type Value = SelectableTemplateVariableItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_value: Option<String> = None;
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut visible_tags: Option<Option<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default_value" => {
                            if v.is_null() {
                                continue;
                            }
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visible_tags" => {
                            visible_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SelectableTemplateVariableItems {
                    default_value,
                    name,
                    prefix,
                    visible_tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SelectableTemplateVariableItemsVisitor)
    }
}

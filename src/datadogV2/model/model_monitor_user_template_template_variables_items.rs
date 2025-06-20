// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of objects representing template variables on the monitor which can have selectable values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorUserTemplateTemplateVariablesItems {
    /// Available values for the variable.
    #[serde(rename = "available_values")]
    pub available_values: Option<Vec<String>>,
    /// Default values of the template variable.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the template variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag key associated with the variable. This works the same as dashboard template variables.
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorUserTemplateTemplateVariablesItems {
    pub fn new(name: String) -> MonitorUserTemplateTemplateVariablesItems {
        MonitorUserTemplateTemplateVariablesItems {
            available_values: None,
            defaults: None,
            name,
            tag_key: None,
            _unparsed: false,
        }
    }

    pub fn available_values(mut self, value: Vec<String>) -> Self {
        self.available_values = Some(value);
        self
    }

    pub fn defaults(mut self, value: Vec<String>) -> Self {
        self.defaults = Some(value);
        self
    }

    pub fn tag_key(mut self, value: String) -> Self {
        self.tag_key = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorUserTemplateTemplateVariablesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorUserTemplateTemplateVariablesItemsVisitor;
        impl<'a> Visitor<'a> for MonitorUserTemplateTemplateVariablesItemsVisitor {
            type Value = MonitorUserTemplateTemplateVariablesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut available_values: Option<Vec<String>> = None;
                let mut defaults: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut tag_key: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "available_values" => {
                            if v.is_null() {
                                continue;
                            }
                            available_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaults" => {
                            if v.is_null() {
                                continue;
                            }
                            defaults = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = MonitorUserTemplateTemplateVariablesItems {
                    available_values,
                    defaults,
                    name,
                    tag_key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorUserTemplateTemplateVariablesItemsVisitor)
    }
}

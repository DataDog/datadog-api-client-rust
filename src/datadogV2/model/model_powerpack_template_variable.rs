// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack template variables.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackTemplateVariable {
    /// One or many template variable default values within the saved view, which are unioned together using `OR` if more than one is specified.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackTemplateVariable {
    pub fn new(name: String) -> PowerpackTemplateVariable {
        PowerpackTemplateVariable {
            defaults: None,
            name,
            _unparsed: false,
        }
    }

    pub fn defaults(mut self, value: Vec<String>) -> Self {
        self.defaults = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackTemplateVariableVisitor;
        impl<'a> Visitor<'a> for PowerpackTemplateVariableVisitor {
            type Value = PowerpackTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut defaults: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "defaults" => {
                            if v.is_null() {
                                continue;
                            }
                            defaults = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = PowerpackTemplateVariable {
                    defaults,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackTemplateVariableVisitor)
    }
}

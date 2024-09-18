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
    /// The list of values that the template variable drop-down is limited to.
    #[serde(
        rename = "available_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub available_values: Option<Option<Vec<String>>>,
    /// One or many template variable default values within the saved view, which are unioned together using `OR` if more than one is specified.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag prefix associated with the variable. Only tags with this prefix appear in the variable drop-down.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option")]
    pub prefix: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackTemplateVariable {
    pub fn new(name: String) -> PowerpackTemplateVariable {
        PowerpackTemplateVariable {
            available_values: None,
            defaults: None,
            name,
            prefix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn available_values(mut self, value: Option<Vec<String>>) -> Self {
        self.available_values = Some(value);
        self
    }

    pub fn defaults(mut self, value: Vec<String>) -> Self {
        self.defaults = Some(value);
        self
    }

    pub fn prefix(mut self, value: Option<String>) -> Self {
        self.prefix = Some(value);
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
                let mut available_values: Option<Option<Vec<String>>> = None;
                let mut defaults: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut prefix: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "available_values" => {
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
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = PowerpackTemplateVariable {
                    available_values,
                    defaults,
                    name,
                    prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackTemplateVariableVisitor)
    }
}

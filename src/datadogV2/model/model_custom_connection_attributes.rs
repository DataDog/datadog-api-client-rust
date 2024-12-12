// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CustomConnectionAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomConnectionAttributes {
    /// The `attributes` `name`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The definition of `CustomConnectionAttributesOnPremRunner` object.
    #[serde(rename = "onPremRunner")]
    pub on_prem_runner: Option<crate::datadogV2::model::CustomConnectionAttributesOnPremRunner>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomConnectionAttributes {
    pub fn new() -> CustomConnectionAttributes {
        CustomConnectionAttributes {
            name: None,
            on_prem_runner: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn on_prem_runner(
        mut self,
        value: crate::datadogV2::model::CustomConnectionAttributesOnPremRunner,
    ) -> Self {
        self.on_prem_runner = Some(value);
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

impl Default for CustomConnectionAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomConnectionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomConnectionAttributesVisitor;
        impl<'a> Visitor<'a> for CustomConnectionAttributesVisitor {
            type Value = CustomConnectionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut on_prem_runner: Option<
                    crate::datadogV2::model::CustomConnectionAttributesOnPremRunner,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "onPremRunner" => {
                            if v.is_null() {
                                continue;
                            }
                            on_prem_runner =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomConnectionAttributes {
                    name,
                    on_prem_runner,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomConnectionAttributesVisitor)
    }
}

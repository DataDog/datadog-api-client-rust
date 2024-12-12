// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AppBuilderErrorErrorsItemsSource` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AppBuilderErrorErrorsItemsSource {
    /// The `source` `parameter`.
    #[serde(rename = "parameter")]
    pub parameter: Option<String>,
    /// The `source` `pointer`.
    #[serde(rename = "pointer")]
    pub pointer: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AppBuilderErrorErrorsItemsSource {
    pub fn new() -> AppBuilderErrorErrorsItemsSource {
        AppBuilderErrorErrorsItemsSource {
            parameter: None,
            pointer: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn parameter(mut self, value: String) -> Self {
        self.parameter = Some(value);
        self
    }

    pub fn pointer(mut self, value: String) -> Self {
        self.pointer = Some(value);
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

impl Default for AppBuilderErrorErrorsItemsSource {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AppBuilderErrorErrorsItemsSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AppBuilderErrorErrorsItemsSourceVisitor;
        impl<'a> Visitor<'a> for AppBuilderErrorErrorsItemsSourceVisitor {
            type Value = AppBuilderErrorErrorsItemsSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut parameter: Option<String> = None;
                let mut pointer: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "parameter" => {
                            if v.is_null() {
                                continue;
                            }
                            parameter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pointer" => {
                            if v.is_null() {
                                continue;
                            }
                            pointer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AppBuilderErrorErrorsItemsSource {
                    parameter,
                    pointer,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AppBuilderErrorErrorsItemsSourceVisitor)
    }
}

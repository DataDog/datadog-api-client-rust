// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// References to the source of the error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JSONAPIErrorItemSource {
    /// A string indicating the name of a single request header which caused the error.
    #[serde(rename = "header")]
    pub header: Option<String>,
    /// A string indicating which URI query parameter caused the error.
    #[serde(rename = "parameter")]
    pub parameter: Option<String>,
    /// A JSON pointer to the value in the request document that caused the error.
    #[serde(rename = "pointer")]
    pub pointer: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JSONAPIErrorItemSource {
    pub fn new() -> JSONAPIErrorItemSource {
        JSONAPIErrorItemSource {
            header: None,
            parameter: None,
            pointer: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn header(mut self, value: String) -> Self {
        self.header = Some(value);
        self
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

impl Default for JSONAPIErrorItemSource {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for JSONAPIErrorItemSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JSONAPIErrorItemSourceVisitor;
        impl<'a> Visitor<'a> for JSONAPIErrorItemSourceVisitor {
            type Value = JSONAPIErrorItemSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut header: Option<String> = None;
                let mut parameter: Option<String> = None;
                let mut pointer: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "header" => {
                            if v.is_null() {
                                continue;
                            }
                            header = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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

                let content = JSONAPIErrorItemSource {
                    header,
                    parameter,
                    pointer,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JSONAPIErrorItemSourceVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Code vulnerability location.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CodeLocation {
    /// Vulnerability location file path.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    /// Vulnerability extracted location.
    #[serde(rename = "location")]
    pub location: String,
    /// Vulnerability location method.
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CodeLocation {
    pub fn new(location: String) -> CodeLocation {
        CodeLocation {
            file_path: None,
            location,
            method: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    pub fn method(mut self, value: String) -> Self {
        self.method = Some(value);
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

impl<'de> Deserialize<'de> for CodeLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CodeLocationVisitor;
        impl<'a> Visitor<'a> for CodeLocationVisitor {
            type Value = CodeLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file_path: Option<String> = None;
                let mut location: Option<String> = None;
                let mut method: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            if v.is_null() {
                                continue;
                            }
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let location = location.ok_or_else(|| M::Error::missing_field("location"))?;

                let content = CodeLocation {
                    file_path,
                    location,
                    method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CodeLocationVisitor)
    }
}

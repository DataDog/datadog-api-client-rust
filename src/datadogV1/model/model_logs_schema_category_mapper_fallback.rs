// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used to override hardcoded category values with a value pulled from a source attribute on the log.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsSchemaCategoryMapperFallback {
    /// Fallback sources used to populate value of field.
    #[serde(rename = "sources")]
    pub sources: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// Values that define when the fallback is used.
    #[serde(rename = "values")]
    pub values: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsSchemaCategoryMapperFallback {
    pub fn new() -> LogsSchemaCategoryMapperFallback {
        LogsSchemaCategoryMapperFallback {
            sources: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn sources(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn values(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.values = Some(value);
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

impl Default for LogsSchemaCategoryMapperFallback {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsSchemaCategoryMapperFallback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsSchemaCategoryMapperFallbackVisitor;
        impl<'a> Visitor<'a> for LogsSchemaCategoryMapperFallbackVisitor {
            type Value = LogsSchemaCategoryMapperFallback;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut sources: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut values: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsSchemaCategoryMapperFallback {
                    sources,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsSchemaCategoryMapperFallbackVisitor)
    }
}

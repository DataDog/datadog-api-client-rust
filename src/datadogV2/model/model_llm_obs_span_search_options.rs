// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Additional options for a span search request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsSpanSearchOptions {
    /// Whether to include attachment data in the response. Defaults to `true`.
    #[serde(rename = "include_attachments")]
    pub include_attachments: Option<bool>,
    /// Offset in seconds applied to both `from` and `to` timestamps.
    #[serde(rename = "time_offset")]
    pub time_offset: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsSpanSearchOptions {
    pub fn new() -> LLMObsSpanSearchOptions {
        LLMObsSpanSearchOptions {
            include_attachments: None,
            time_offset: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_attachments(mut self, value: bool) -> Self {
        self.include_attachments = Some(value);
        self
    }

    pub fn time_offset(mut self, value: i64) -> Self {
        self.time_offset = Some(value);
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

impl Default for LLMObsSpanSearchOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsSpanSearchOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsSpanSearchOptionsVisitor;
        impl<'a> Visitor<'a> for LLMObsSpanSearchOptionsVisitor {
            type Value = LLMObsSpanSearchOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_attachments: Option<bool> = None;
                let mut time_offset: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_attachments" => {
                            if v.is_null() {
                                continue;
                            }
                            include_attachments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            time_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsSpanSearchOptions {
                    include_attachments,
                    time_offset,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsSpanSearchOptionsVisitor)
    }
}

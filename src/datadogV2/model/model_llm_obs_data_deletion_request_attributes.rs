// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an LLM Observability data deletion request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDataDeletionRequestAttributes {
    /// Optional delay in seconds before the deletion is executed.
    #[serde(rename = "delay")]
    pub delay: Option<i64>,
    /// Start of the deletion time range in milliseconds since Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// Query filters selecting the data to delete. Must include a `query` key with an `@trace_id` filter.
    #[serde(rename = "query")]
    pub query: std::collections::BTreeMap<String, String>,
    /// End of the deletion time range in milliseconds since Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDataDeletionRequestAttributes {
    pub fn new(
        from: i64,
        query: std::collections::BTreeMap<String, String>,
        to: i64,
    ) -> LLMObsDataDeletionRequestAttributes {
        LLMObsDataDeletionRequestAttributes {
            delay: None,
            from,
            query,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn delay(mut self, value: i64) -> Self {
        self.delay = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsDataDeletionRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDataDeletionRequestAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsDataDeletionRequestAttributesVisitor {
            type Value = LLMObsDataDeletionRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delay: Option<i64> = None;
                let mut from: Option<i64> = None;
                let mut query: Option<std::collections::BTreeMap<String, String>> = None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "delay" => {
                            if v.is_null() {
                                continue;
                            }
                            delay = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = LLMObsDataDeletionRequestAttributes {
                    delay,
                    from,
                    query,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDataDeletionRequestAttributesVisitor)
    }
}

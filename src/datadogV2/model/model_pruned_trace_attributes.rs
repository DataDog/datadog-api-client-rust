// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a pruned trace returned by the Get pruned trace by ID endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PrunedTraceAttributes {
    /// Indicates whether the underlying trace was truncated because its size
    /// exceeded the maximum that can be retrieved from storage.
    #[serde(rename = "is_truncated")]
    pub is_truncated: bool,
    /// The size, in bytes, of the original (non-pruned) trace before summarization.
    #[serde(rename = "size_bytes")]
    pub size_bytes: i32,
    /// A summarized, hierarchical view of a trace.
    #[serde(rename = "summarized_trace")]
    pub summarized_trace: crate::datadogV2::model::SummarizedTrace,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PrunedTraceAttributes {
    pub fn new(
        is_truncated: bool,
        size_bytes: i32,
        summarized_trace: crate::datadogV2::model::SummarizedTrace,
    ) -> PrunedTraceAttributes {
        PrunedTraceAttributes {
            is_truncated,
            size_bytes,
            summarized_trace,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for PrunedTraceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrunedTraceAttributesVisitor;
        impl<'a> Visitor<'a> for PrunedTraceAttributesVisitor {
            type Value = PrunedTraceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_truncated: Option<bool> = None;
                let mut size_bytes: Option<i32> = None;
                let mut summarized_trace: Option<crate::datadogV2::model::SummarizedTrace> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_truncated" => {
                            is_truncated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size_bytes" => {
                            size_bytes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summarized_trace" => {
                            summarized_trace =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_truncated =
                    is_truncated.ok_or_else(|| M::Error::missing_field("is_truncated"))?;
                let size_bytes = size_bytes.ok_or_else(|| M::Error::missing_field("size_bytes"))?;
                let summarized_trace =
                    summarized_trace.ok_or_else(|| M::Error::missing_field("summarized_trace"))?;

                let content = PrunedTraceAttributes {
                    is_truncated,
                    size_bytes,
                    summarized_trace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PrunedTraceAttributesVisitor)
    }
}

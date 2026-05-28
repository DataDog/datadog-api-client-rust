// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A summarized, hierarchical view of a trace.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SummarizedTrace {
    /// A node in the pruned trace tree.
    #[serde(rename = "root")]
    pub root: crate::datadogV2::model::SummarizedSpan,
    /// The full 128-bit trace ID, encoded as a 32-character hexadecimal string.
    #[serde(rename = "traceId")]
    pub trace_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SummarizedTrace {
    pub fn new(root: crate::datadogV2::model::SummarizedSpan, trace_id: String) -> SummarizedTrace {
        SummarizedTrace {
            root,
            trace_id,
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

impl<'de> Deserialize<'de> for SummarizedTrace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SummarizedTraceVisitor;
        impl<'a> Visitor<'a> for SummarizedTraceVisitor {
            type Value = SummarizedTrace;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut root: Option<crate::datadogV2::model::SummarizedSpan> = None;
                let mut trace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "root" => {
                            root = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceId" => {
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let root = root.ok_or_else(|| M::Error::missing_field("root"))?;
                let trace_id = trace_id.ok_or_else(|| M::Error::missing_field("trace_id"))?;

                let content = SummarizedTrace {
                    root,
                    trace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SummarizedTraceVisitor)
    }
}

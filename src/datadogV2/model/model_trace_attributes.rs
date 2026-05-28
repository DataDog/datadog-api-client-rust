// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a trace returned by the Get trace by ID endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TraceAttributes {
    /// Indicates whether the trace was truncated because its size exceeded the maximum response payload.
    #[serde(rename = "is_truncated")]
    pub is_truncated: bool,
    /// The list of spans that compose the trace.
    #[serde(rename = "spans")]
    pub spans: Vec<crate::datadogV2::model::APMTraceSpan>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TraceAttributes {
    pub fn new(
        is_truncated: bool,
        spans: Vec<crate::datadogV2::model::APMTraceSpan>,
    ) -> TraceAttributes {
        TraceAttributes {
            is_truncated,
            spans,
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

impl<'de> Deserialize<'de> for TraceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TraceAttributesVisitor;
        impl<'a> Visitor<'a> for TraceAttributesVisitor {
            type Value = TraceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_truncated: Option<bool> = None;
                let mut spans: Option<Vec<crate::datadogV2::model::APMTraceSpan>> = None;
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
                        "spans" => {
                            spans = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let spans = spans.ok_or_else(|| M::Error::missing_field("spans"))?;

                let content = TraceAttributes {
                    is_truncated,
                    spans,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TraceAttributesVisitor)
    }
}

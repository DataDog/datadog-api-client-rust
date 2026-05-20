// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filter criteria for an LLM Observability span search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsSpanFilter {
    /// Start of the time range. Accepts ISO 8601 or relative format (e.g., `now-15m`). Defaults to `now-15m`.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// Filter by ML application name.
    #[serde(rename = "ml_app")]
    pub ml_app: Option<String>,
    /// Search query using LLM Observability query syntax. Supports attribute filters using the field:value syntax (e.g. session_id, trace_id, ml_app, meta.span.kind). When provided, structured field filters (`span_id`, `trace_id`, etc.) are ignored.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Filter by exact span ID.
    #[serde(rename = "span_id")]
    pub span_id: Option<String>,
    /// Filter by span kind (e.g., llm, agent, tool, task, workflow).
    #[serde(rename = "span_kind")]
    pub span_kind: Option<String>,
    /// Filter by span name.
    #[serde(rename = "span_name")]
    pub span_name: Option<String>,
    /// Filter by tag key-value pairs.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// End of the time range. Accepts ISO 8601 or relative format (e.g., `now`). Defaults to `now`.
    #[serde(rename = "to")]
    pub to: Option<String>,
    /// Filter by exact trace ID.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsSpanFilter {
    pub fn new() -> LLMObsSpanFilter {
        LLMObsSpanFilter {
            from: None,
            ml_app: None,
            query: None,
            span_id: None,
            span_kind: None,
            span_name: None,
            tags: None,
            to: None,
            trace_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn ml_app(mut self, value: String) -> Self {
        self.ml_app = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn span_id(mut self, value: String) -> Self {
        self.span_id = Some(value);
        self
    }

    pub fn span_kind(mut self, value: String) -> Self {
        self.span_kind = Some(value);
        self
    }

    pub fn span_name(mut self, value: String) -> Self {
        self.span_name = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
        self
    }

    pub fn trace_id(mut self, value: String) -> Self {
        self.trace_id = Some(value);
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

impl Default for LLMObsSpanFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsSpanFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsSpanFilterVisitor;
        impl<'a> Visitor<'a> for LLMObsSpanFilterVisitor {
            type Value = LLMObsSpanFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<String> = None;
                let mut ml_app: Option<String> = None;
                let mut query: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut span_kind: Option<String> = None;
                let mut span_name: Option<String> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut to: Option<String> = None;
                let mut trace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_app" => {
                            if v.is_null() {
                                continue;
                            }
                            ml_app = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            if v.is_null() {
                                continue;
                            }
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_kind" => {
                            if v.is_null() {
                                continue;
                            }
                            span_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_name" => {
                            if v.is_null() {
                                continue;
                            }
                            span_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsSpanFilter {
                    from,
                    ml_app,
                    query,
                    span_id,
                    span_kind,
                    span_name,
                    tags,
                    to,
                    trace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsSpanFilterVisitor)
    }
}

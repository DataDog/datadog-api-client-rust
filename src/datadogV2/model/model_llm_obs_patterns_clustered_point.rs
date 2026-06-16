// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single data point grouped into a topic.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsClusteredPoint {
    /// Identifier of the source event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// Unique identifier of the clustered point.
    #[serde(rename = "id")]
    pub id: String,
    /// Input text of the source span.
    #[serde(rename = "input")]
    pub input: String,
    /// Whether the point is included in the patterns dataset.
    #[serde(rename = "is_included")]
    pub is_included: bool,
    /// Whether the point is suggested for inclusion in the patterns dataset.
    #[serde(rename = "is_suggested")]
    pub is_suggested: bool,
    /// Identifier of the source session.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Identifier of the source span.
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// Identifier of the topic the point belongs to.
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsClusteredPoint {
    pub fn new(
        event_id: String,
        id: String,
        input: String,
        is_included: bool,
        is_suggested: bool,
        session_id: String,
        span_id: String,
        topic_id: String,
    ) -> LLMObsPatternsClusteredPoint {
        LLMObsPatternsClusteredPoint {
            event_id,
            id,
            input,
            is_included,
            is_suggested,
            session_id,
            span_id,
            topic_id,
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

impl<'de> Deserialize<'de> for LLMObsPatternsClusteredPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsClusteredPointVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsClusteredPointVisitor {
            type Value = LLMObsPatternsClusteredPoint;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_id: Option<String> = None;
                let mut id: Option<String> = None;
                let mut input: Option<String> = None;
                let mut is_included: Option<bool> = None;
                let mut is_suggested: Option<bool> = None;
                let mut session_id: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut topic_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_id" => {
                            event_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input" => {
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_included" => {
                            is_included =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_suggested" => {
                            is_suggested =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "topic_id" => {
                            topic_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let event_id = event_id.ok_or_else(|| M::Error::missing_field("event_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;
                let is_included =
                    is_included.ok_or_else(|| M::Error::missing_field("is_included"))?;
                let is_suggested =
                    is_suggested.ok_or_else(|| M::Error::missing_field("is_suggested"))?;
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let topic_id = topic_id.ok_or_else(|| M::Error::missing_field("topic_id"))?;

                let content = LLMObsPatternsClusteredPoint {
                    event_id,
                    id,
                    input,
                    is_included,
                    is_suggested,
                    session_id,
                    span_id,
                    topic_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsClusteredPointVisitor)
    }
}

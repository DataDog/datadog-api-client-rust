// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single message in a span input or output.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsSpanMessage {
    /// Text content of the message.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// Unique identifier of the message.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Role of the message sender (e.g., user, assistant, system).
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// Tool calls made in this message.
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<crate::datadogV2::model::LLMObsSpanToolCall>>,
    /// Tool results returned in this message.
    #[serde(rename = "tool_results")]
    pub tool_results: Option<Vec<crate::datadogV2::model::LLMObsSpanToolResult>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsSpanMessage {
    pub fn new() -> LLMObsSpanMessage {
        LLMObsSpanMessage {
            content: None,
            id: None,
            role: None,
            tool_calls: None,
            tool_results: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn role(mut self, value: String) -> Self {
        self.role = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<crate::datadogV2::model::LLMObsSpanToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn tool_results(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsSpanToolResult>,
    ) -> Self {
        self.tool_results = Some(value);
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

impl Default for LLMObsSpanMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsSpanMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsSpanMessageVisitor;
        impl<'a> Visitor<'a> for LLMObsSpanMessageVisitor {
            type Value = LLMObsSpanMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut id: Option<String> = None;
                let mut role: Option<String> = None;
                let mut tool_calls: Option<Vec<crate::datadogV2::model::LLMObsSpanToolCall>> = None;
                let mut tool_results: Option<Vec<crate::datadogV2::model::LLMObsSpanToolResult>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_calls" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_calls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_results" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_results =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsSpanMessage {
                    content,
                    id,
                    role,
                    tool_calls,
                    tool_results,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsSpanMessageVisitor)
    }
}

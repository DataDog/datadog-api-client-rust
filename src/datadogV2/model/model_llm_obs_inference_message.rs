// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single message in an LLM inference conversation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsInferenceMessage {
    /// Plain text content of the message.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// List of structured content blocks in a message.
    #[serde(rename = "contents")]
    pub contents: Option<Vec<crate::datadogV2::model::LLMObsInferenceContent>>,
    /// Unique identifier for the message.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The role of the message author.
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// List of tool calls in a message.
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<crate::datadogV2::model::LLMObsInferenceToolCall>>,
    /// List of tool results in a message.
    #[serde(rename = "tool_results")]
    pub tool_results: Option<Vec<crate::datadogV2::model::LLMObsInferenceToolResult>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsInferenceMessage {
    pub fn new() -> LLMObsInferenceMessage {
        LLMObsInferenceMessage {
            content: None,
            contents: None,
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

    pub fn contents(mut self, value: Vec<crate::datadogV2::model::LLMObsInferenceContent>) -> Self {
        self.contents = Some(value);
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

    pub fn tool_calls(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsInferenceToolCall>,
    ) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn tool_results(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsInferenceToolResult>,
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

impl Default for LLMObsInferenceMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsInferenceMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsInferenceMessageVisitor;
        impl<'a> Visitor<'a> for LLMObsInferenceMessageVisitor {
            type Value = LLMObsInferenceMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut contents: Option<Vec<crate::datadogV2::model::LLMObsInferenceContent>> =
                    None;
                let mut id: Option<String> = None;
                let mut role: Option<String> = None;
                let mut tool_calls: Option<Vec<crate::datadogV2::model::LLMObsInferenceToolCall>> =
                    None;
                let mut tool_results: Option<
                    Vec<crate::datadogV2::model::LLMObsInferenceToolResult>,
                > = None;
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
                        "contents" => {
                            if v.is_null() {
                                continue;
                            }
                            contents = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = LLMObsInferenceMessage {
                    content,
                    contents,
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

        deserializer.deserialize_any(LLMObsInferenceMessageVisitor)
    }
}

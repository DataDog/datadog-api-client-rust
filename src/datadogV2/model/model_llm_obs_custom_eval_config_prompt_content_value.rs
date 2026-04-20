// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Value of a prompt message content block.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigPromptContentValue {
    /// Text content of the message block.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// A tool call within a prompt message.
    #[serde(rename = "tool_call")]
    pub tool_call: Option<crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolCall>,
    /// A tool call result within a prompt message.
    #[serde(rename = "tool_call_result")]
    pub tool_call_result: Option<crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolResult>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigPromptContentValue {
    pub fn new() -> LLMObsCustomEvalConfigPromptContentValue {
        LLMObsCustomEvalConfigPromptContentValue {
            text: None,
            tool_call: None,
            tool_call_result: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_call(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolCall,
    ) -> Self {
        self.tool_call = Some(value);
        self
    }

    pub fn tool_call_result(
        mut self,
        value: crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolResult,
    ) -> Self {
        self.tool_call_result = Some(value);
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

impl Default for LLMObsCustomEvalConfigPromptContentValue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigPromptContentValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigPromptContentValueVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigPromptContentValueVisitor {
            type Value = LLMObsCustomEvalConfigPromptContentValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut text: Option<String> = None;
                let mut tool_call: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolCall,
                > = None;
                let mut tool_call_result: Option<
                    crate::datadogV2::model::LLMObsCustomEvalConfigPromptToolResult,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "text" => {
                            if v.is_null() {
                                continue;
                            }
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_call" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_call = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_call_result" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_call_result =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsCustomEvalConfigPromptContentValue {
                    text,
                    tool_call,
                    tool_call_result,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigPromptContentValueVisitor)
    }
}

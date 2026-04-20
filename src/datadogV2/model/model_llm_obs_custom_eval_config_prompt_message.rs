// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A message in the prompt template for a custom LLM judge evaluator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigPromptMessage {
    /// Text content of the message.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// Multi-part content blocks for the message.
    #[serde(rename = "contents")]
    pub contents: Option<Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptContent>>,
    /// Role of the message author.
    #[serde(rename = "role")]
    pub role: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigPromptMessage {
    pub fn new(role: String) -> LLMObsCustomEvalConfigPromptMessage {
        LLMObsCustomEvalConfigPromptMessage {
            content: None,
            contents: None,
            role,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    pub fn contents(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptContent>,
    ) -> Self {
        self.contents = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigPromptMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigPromptMessageVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigPromptMessageVisitor {
            type Value = LLMObsCustomEvalConfigPromptMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut contents: Option<
                    Vec<crate::datadogV2::model::LLMObsCustomEvalConfigPromptContent>,
                > = None;
                let mut role: Option<String> = None;
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
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;

                let content = LLMObsCustomEvalConfigPromptMessage {
                    content,
                    contents,
                    role,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigPromptMessageVisitor)
    }
}

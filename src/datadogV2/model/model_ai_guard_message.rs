// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single message in the conversation to evaluate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardMessage {
    /// The message content, either a plain string or an array of content parts.
    #[serde(rename = "content")]
    pub content: Option<crate::datadogV2::model::AIGuardMessageContent>,
    /// The role of the message author in the conversation.
    #[serde(rename = "role")]
    pub role: crate::datadogV2::model::AIGuardMessageRole,
    /// The ID of the tool call this message is responding to, required for tool messages.
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: Option<String>,
    /// Tool calls issued by the assistant in this message.
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<crate::datadogV2::model::AIGuardToolCall>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardMessage {
    pub fn new(role: crate::datadogV2::model::AIGuardMessageRole) -> AIGuardMessage {
        AIGuardMessage {
            content: None,
            role,
            tool_call_id: None,
            tool_calls: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: crate::datadogV2::model::AIGuardMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: String) -> Self {
        self.tool_call_id = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<crate::datadogV2::model::AIGuardToolCall>) -> Self {
        self.tool_calls = Some(value);
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

impl<'de> Deserialize<'de> for AIGuardMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardMessageVisitor;
        impl<'a> Visitor<'a> for AIGuardMessageVisitor {
            type Value = AIGuardMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<crate::datadogV2::model::AIGuardMessageContent> = None;
                let mut role: Option<crate::datadogV2::model::AIGuardMessageRole> = None;
                let mut tool_call_id: Option<String> = None;
                let mut tool_calls: Option<Vec<crate::datadogV2::model::AIGuardToolCall>> = None;
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
                            if let Some(ref _content) = content {
                                match _content {
                                    crate::datadogV2::model::AIGuardMessageContent::UnparsedObject(_content) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _role) = role {
                                match _role {
                                    crate::datadogV2::model::AIGuardMessageRole::UnparsedObject(
                                        _role,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tool_call_id" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_call_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_calls" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_calls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;

                let content = AIGuardMessage {
                    content,
                    role,
                    tool_call_id,
                    tool_calls,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardMessageVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChatMessage {
    /// The chat ID.
    #[serde(rename = "chatId")]
    pub chat_id: String,
    /// The message content.
    #[serde(rename = "content")]
    pub content: String,
    /// The message ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The role of the message sender.
    #[serde(rename = "role")]
    pub role: crate::datadogV2::model::ChatMessageRole,
    /// The UUID of the user who sent the message.
    #[serde(rename = "userUuid")]
    pub user_uuid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChatMessage {
    pub fn new(
        chat_id: String,
        content: String,
        id: String,
        role: crate::datadogV2::model::ChatMessageRole,
    ) -> ChatMessage {
        ChatMessage {
            chat_id,
            content,
            id,
            role,
            user_uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn user_uuid(mut self, value: String) -> Self {
        self.user_uuid = Some(value);
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

impl<'de> Deserialize<'de> for ChatMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChatMessageVisitor;
        impl<'a> Visitor<'a> for ChatMessageVisitor {
            type Value = ChatMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chat_id: Option<String> = None;
                let mut content: Option<String> = None;
                let mut id: Option<String> = None;
                let mut role: Option<crate::datadogV2::model::ChatMessageRole> = None;
                let mut user_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chatId" => {
                            chat_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _role) = role {
                                match _role {
                                    crate::datadogV2::model::ChatMessageRole::UnparsedObject(
                                        _role,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "userUuid" => {
                            if v.is_null() {
                                continue;
                            }
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let chat_id = chat_id.ok_or_else(|| M::Error::missing_field("chat_id"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;

                let content = ChatMessage {
                    chat_id,
                    content,
                    id,
                    role,
                    user_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChatMessageVisitor)
    }
}

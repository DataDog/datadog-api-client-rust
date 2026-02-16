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
pub struct DataTransformationRequest {
    /// Previous chat messages for iterative interaction.
    #[serde(rename = "chatHistory")]
    pub chat_history: Option<Vec<crate::datadogV2::model::ChatHistoryItem>>,
    #[serde(rename = "context")]
    pub context: Option<crate::datadogV2::model::DataTransformationContext>,
    /// The programming language for the transformation.
    #[serde(rename = "language")]
    pub language: Option<crate::datadogV2::model::DataTransformationLanguage>,
    /// Whether to stream the response.
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    /// The user's prompt describing the desired transformation.
    #[serde(rename = "userPrompt")]
    pub user_prompt: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataTransformationRequest {
    pub fn new(user_prompt: String) -> DataTransformationRequest {
        DataTransformationRequest {
            chat_history: None,
            context: None,
            language: None,
            stream: None,
            user_prompt,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chat_history(mut self, value: Vec<crate::datadogV2::model::ChatHistoryItem>) -> Self {
        self.chat_history = Some(value);
        self
    }

    pub fn context(mut self, value: crate::datadogV2::model::DataTransformationContext) -> Self {
        self.context = Some(value);
        self
    }

    pub fn language(mut self, value: crate::datadogV2::model::DataTransformationLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
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

impl<'de> Deserialize<'de> for DataTransformationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataTransformationRequestVisitor;
        impl<'a> Visitor<'a> for DataTransformationRequestVisitor {
            type Value = DataTransformationRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chat_history: Option<Vec<crate::datadogV2::model::ChatHistoryItem>> = None;
                let mut context: Option<crate::datadogV2::model::DataTransformationContext> = None;
                let mut language: Option<crate::datadogV2::model::DataTransformationLanguage> =
                    None;
                let mut stream: Option<bool> = None;
                let mut user_prompt: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chatHistory" => {
                            if v.is_null() {
                                continue;
                            }
                            chat_history =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _language) = language {
                                match _language {
                                    crate::datadogV2::model::DataTransformationLanguage::UnparsedObject(_language) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "stream" => {
                            if v.is_null() {
                                continue;
                            }
                            stream = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userPrompt" => {
                            user_prompt =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let user_prompt =
                    user_prompt.ok_or_else(|| M::Error::missing_field("user_prompt"))?;

                let content = DataTransformationRequest {
                    chat_history,
                    context,
                    language,
                    stream,
                    user_prompt,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataTransformationRequestVisitor)
    }
}

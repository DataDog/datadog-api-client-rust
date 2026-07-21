// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a flattened prompt version returned for SDK consumption. Exactly one of `template` and `chat_template` is returned.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPromptSDKDataAttributes {
    /// Chat template for this prompt version, as a list of role and content messages. Omitted for text templates.
    #[serde(rename = "chat_template")]
    pub chat_template: Option<Vec<crate::datadogV2::model::LLMObsPromptChatMessage>>,
    /// Labels attached to the selected version.
    #[deprecated]
    #[serde(rename = "labels")]
    pub labels: Option<Vec<String>>,
    /// Customer-provided identifier of the prompt.
    #[serde(rename = "prompt_id")]
    pub prompt_id: Option<String>,
    /// Unique identifier of this prompt version.
    #[serde(rename = "prompt_version_uuid")]
    pub prompt_version_uuid: Option<String>,
    /// Text template for this prompt version. Omitted for chat templates.
    #[serde(rename = "template")]
    pub template: Option<String>,
    /// Version identifier for this prompt version. This is the sequential version number unless a user-supplied version identifier was set, in which case that identifier is used instead.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPromptSDKDataAttributes {
    pub fn new() -> LLMObsPromptSDKDataAttributes {
        #[allow(deprecated)]
        LLMObsPromptSDKDataAttributes {
            chat_template: None,
            labels: None,
            prompt_id: None,
            prompt_version_uuid: None,
            template: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn chat_template(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsPromptChatMessage>,
    ) -> Self {
        self.chat_template = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn prompt_id(mut self, value: String) -> Self {
        self.prompt_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn prompt_version_uuid(mut self, value: String) -> Self {
        self.prompt_version_uuid = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn template(mut self, value: String) -> Self {
        self.template = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl Default for LLMObsPromptSDKDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsPromptSDKDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPromptSDKDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPromptSDKDataAttributesVisitor {
            type Value = LLMObsPromptSDKDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chat_template: Option<
                    Vec<crate::datadogV2::model::LLMObsPromptChatMessage>,
                > = None;
                let mut labels: Option<Vec<String>> = None;
                let mut prompt_id: Option<String> = None;
                let mut prompt_version_uuid: Option<String> = None;
                let mut template: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chat_template" => {
                            if v.is_null() {
                                continue;
                            }
                            chat_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "labels" => {
                            if v.is_null() {
                                continue;
                            }
                            labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_id" => {
                            if v.is_null() {
                                continue;
                            }
                            prompt_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_version_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            prompt_version_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            if v.is_null() {
                                continue;
                            }
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = LLMObsPromptSDKDataAttributes {
                    chat_template,
                    labels,
                    prompt_id,
                    prompt_version_uuid,
                    template,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPromptSDKDataAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for creating the OpenAI LLM Evaluations integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
    /// Comma-separated list of custom tags applied to the data collected for this account, for example `team:ai,env:prod`.
    #[serde(rename = "custom_tags")]
    pub custom_tags: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
    pub fn new() -> OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
        OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
            custom_tags: None,
            _unparsed: false,
        }
    }

    pub fn custom_tags(mut self, value: String) -> Self {
        self.custom_tags = Some(value);
        self
    }
}

impl Default for OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAILLMEvaluationsIntegrationAccountSettingsRequestVisitor;
        impl<'a> Visitor<'a> for OpenAILLMEvaluationsIntegrationAccountSettingsRequestVisitor {
            type Value = OpenAILLMEvaluationsIntegrationAccountSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_tags: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = OpenAILLMEvaluationsIntegrationAccountSettingsRequest {
                    custom_tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAILLMEvaluationsIntegrationAccountSettingsRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OpenAI-specific metadata for an inference request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsOpenAIMetadata {
    /// The reasoning effort level for OpenAI models that support it.
    #[serde(
        rename = "reasoning_effort",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reasoning_effort: Option<Option<crate::datadogV2::model::LLMObsOpenAIReasoningEffort>>,
    /// The verbosity of the reasoning summary.
    #[serde(
        rename = "reasoning_summary",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reasoning_summary: Option<Option<crate::datadogV2::model::LLMObsOpenAIReasoningSummary>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsOpenAIMetadata {
    pub fn new() -> LLMObsOpenAIMetadata {
        LLMObsOpenAIMetadata {
            reasoning_effort: None,
            reasoning_summary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reasoning_effort(
        mut self,
        value: Option<crate::datadogV2::model::LLMObsOpenAIReasoningEffort>,
    ) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn reasoning_summary(
        mut self,
        value: Option<crate::datadogV2::model::LLMObsOpenAIReasoningSummary>,
    ) -> Self {
        self.reasoning_summary = Some(value);
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

impl Default for LLMObsOpenAIMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsOpenAIMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsOpenAIMetadataVisitor;
        impl<'a> Visitor<'a> for LLMObsOpenAIMetadataVisitor {
            type Value = LLMObsOpenAIMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reasoning_effort: Option<
                    Option<crate::datadogV2::model::LLMObsOpenAIReasoningEffort>,
                > = None;
                let mut reasoning_summary: Option<
                    Option<crate::datadogV2::model::LLMObsOpenAIReasoningSummary>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reasoning_effort" => {
                            reasoning_effort =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reasoning_effort) = reasoning_effort {
                                match _reasoning_effort {
                                    Some(crate::datadogV2::model::LLMObsOpenAIReasoningEffort::UnparsedObject(_reasoning_effort)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "reasoning_summary" => {
                            reasoning_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reasoning_summary) = reasoning_summary {
                                match _reasoning_summary {
                                    Some(crate::datadogV2::model::LLMObsOpenAIReasoningSummary::UnparsedObject(_reasoning_summary)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsOpenAIMetadata {
                    reasoning_effort,
                    reasoning_summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsOpenAIMetadataVisitor)
    }
}

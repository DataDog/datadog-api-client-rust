// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The model's internal reasoning or thinking output, if available.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsInternalReasoning {
    /// Number of tokens used for internal reasoning.
    #[serde(
        rename = "reasoning_tokens",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reasoning_tokens: Option<Option<i64>>,
    /// The reasoning text produced by the model.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsInternalReasoning {
    pub fn new(text: String) -> LLMObsInternalReasoning {
        LLMObsInternalReasoning {
            reasoning_tokens: None,
            text,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reasoning_tokens(mut self, value: Option<i64>) -> Self {
        self.reasoning_tokens = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsInternalReasoning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsInternalReasoningVisitor;
        impl<'a> Visitor<'a> for LLMObsInternalReasoningVisitor {
            type Value = LLMObsInternalReasoning;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reasoning_tokens: Option<Option<i64>> = None;
                let mut text: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reasoning_tokens" => {
                            reasoning_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let text = text.ok_or_else(|| M::Error::missing_field("text"))?;

                let content = LLMObsInternalReasoning {
                    reasoning_tokens,
                    text,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsInternalReasoningVisitor)
    }
}

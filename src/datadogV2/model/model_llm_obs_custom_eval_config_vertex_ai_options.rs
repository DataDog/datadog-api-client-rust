// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Google Vertex AI-specific options for LLM provider configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigVertexAIOptions {
    /// Google Cloud region.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// Google Cloud project ID.
    #[serde(rename = "project")]
    pub project: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsCustomEvalConfigVertexAIOptions {
    pub fn new() -> LLMObsCustomEvalConfigVertexAIOptions {
        LLMObsCustomEvalConfigVertexAIOptions {
            location: None,
            project: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }

    pub fn project(mut self, value: String) -> Self {
        self.project = Some(value);
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

impl Default for LLMObsCustomEvalConfigVertexAIOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigVertexAIOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigVertexAIOptionsVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigVertexAIOptionsVisitor {
            type Value = LLMObsCustomEvalConfigVertexAIOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut location: Option<String> = None;
                let mut project: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            if v.is_null() {
                                continue;
                            }
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsCustomEvalConfigVertexAIOptions {
                    location,
                    project,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigVertexAIOptionsVisitor)
    }
}

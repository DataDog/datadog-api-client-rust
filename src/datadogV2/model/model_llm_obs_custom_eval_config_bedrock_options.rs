// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// AWS Bedrock-specific options for LLM provider configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsCustomEvalConfigBedrockOptions {
    /// Bedrock inference profile identifier, such as an application inference profile ARN.
    #[serde(rename = "inference_profile")]
    pub inference_profile: Option<String>,
    /// AWS region for Bedrock.
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LLMObsCustomEvalConfigBedrockOptions {
    pub fn new() -> LLMObsCustomEvalConfigBedrockOptions {
        LLMObsCustomEvalConfigBedrockOptions {
            inference_profile: None,
            region: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn inference_profile(mut self, value: String) -> Self {
        self.inference_profile = Some(value);
        self
    }
    
    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for LLMObsCustomEvalConfigBedrockOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsCustomEvalConfigBedrockOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsCustomEvalConfigBedrockOptionsVisitor;
        impl<'a> Visitor<'a> for LLMObsCustomEvalConfigBedrockOptionsVisitor {
            type Value = LLMObsCustomEvalConfigBedrockOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut inference_profile: Option<String> = None;
                let mut region: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "inference_profile" => {
                            if v.is_null() {
                                continue;
                            }
                            inference_profile = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = LLMObsCustomEvalConfigBedrockOptions {
                    inference_profile,
                    region,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsCustomEvalConfigBedrockOptionsVisitor)
    }
}

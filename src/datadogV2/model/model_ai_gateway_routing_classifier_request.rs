// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Decision model and settings for a classified tier. Omission preserves the saved classifier; null clears it. Newly selected models require provider View.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingClassifierRequest {
    /// Minimum decision confidence. Omission preserves a saved value; new classifiers and explicit null default to 0.5.
    #[serde(
        rename = "confidence_threshold",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub confidence_threshold: Option<Option<f64>>,
    /// Traffic context for the classifier. Omission preserves saved context.
    #[serde(rename = "context")]
    pub context: Option<String>,
    /// Decision model on the provider. Omit together with provider to preserve the saved selection.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// Decision-model provider slug. It must name a provider of type `openai`, `baseten`, or `typesafe`, the provider types that serve a decision model. Omit together with model to preserve the saved selection.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingClassifierRequest {
    pub fn new() -> AiGatewayRoutingClassifierRequest {
        AiGatewayRoutingClassifierRequest {
            confidence_threshold: None,
            context: None,
            model: None,
            provider: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence_threshold(mut self, value: Option<f64>) -> Self {
        self.confidence_threshold = Some(value);
        self
    }

    pub fn context(mut self, value: String) -> Self {
        self.context = Some(value);
        self
    }

    pub fn model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
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

impl Default for AiGatewayRoutingClassifierRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayRoutingClassifierRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingClassifierRequestVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingClassifierRequestVisitor {
            type Value = AiGatewayRoutingClassifierRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_threshold: Option<Option<f64>> = None;
                let mut context: Option<String> = None;
                let mut model: Option<String> = None;
                let mut provider: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_threshold" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            confidence_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model" => {
                            if v.is_null() {
                                continue;
                            }
                            model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayRoutingClassifierRequest {
                    confidence_threshold,
                    context,
                    model,
                    provider,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingClassifierRequestVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Saved classified-tier settings. Provider and model are empty when the caller cannot view the provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingClassifier {
    /// Minimum decision confidence.
    #[serde(rename = "confidence_threshold")]
    pub confidence_threshold: f64,
    /// Traffic context for the classifier.
    #[serde(rename = "context")]
    pub context: Option<String>,
    /// Viewable decision model, or an empty string.
    #[serde(rename = "model")]
    pub model: String,
    /// Viewable decision-model provider slug, or an empty string.
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingClassifier {
    pub fn new(
        confidence_threshold: f64,
        model: String,
        provider: String,
    ) -> AiGatewayRoutingClassifier {
        AiGatewayRoutingClassifier {
            confidence_threshold,
            context: None,
            model,
            provider,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn context(mut self, value: String) -> Self {
        self.context = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRoutingClassifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingClassifierVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingClassifierVisitor {
            type Value = AiGatewayRoutingClassifier;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_threshold: Option<f64> = None;
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
                            model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let confidence_threshold = confidence_threshold
                    .ok_or_else(|| M::Error::missing_field("confidence_threshold"))?;
                let model = model.ok_or_else(|| M::Error::missing_field("model"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;

                let content = AiGatewayRoutingClassifier {
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

        deserializer.deserialize_any(AiGatewayRoutingClassifierVisitor)
    }
}

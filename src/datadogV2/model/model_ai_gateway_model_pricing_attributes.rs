// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a model pricing resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingAttributes {
    /// Effective source of the model pricing data.
    #[serde(rename = "effective_source")]
    pub effective_source: Option<crate::datadogV2::model::AiGatewayModelPricingEffectiveSource>,
    /// Pricing configuration for a model.
    #[serde(rename = "pricing_config")]
    pub pricing_config: Option<crate::datadogV2::model::AiGatewayModelPricingConfig>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingAttributes {
    pub fn new() -> AiGatewayModelPricingAttributes {
        AiGatewayModelPricingAttributes {
            effective_source: None,
            pricing_config: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn effective_source(
        mut self,
        value: crate::datadogV2::model::AiGatewayModelPricingEffectiveSource,
    ) -> Self {
        self.effective_source = Some(value);
        self
    }

    pub fn pricing_config(
        mut self,
        value: crate::datadogV2::model::AiGatewayModelPricingConfig,
    ) -> Self {
        self.pricing_config = Some(value);
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

impl Default for AiGatewayModelPricingAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingAttributesVisitor {
            type Value = AiGatewayModelPricingAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut effective_source: Option<
                    crate::datadogV2::model::AiGatewayModelPricingEffectiveSource,
                > = None;
                let mut pricing_config: Option<
                    crate::datadogV2::model::AiGatewayModelPricingConfig,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "effective_source" => {
                            if v.is_null() {
                                continue;
                            }
                            effective_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _effective_source) = effective_source {
                                match _effective_source {
                                    crate::datadogV2::model::AiGatewayModelPricingEffectiveSource::UnparsedObject(_effective_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "pricing_config" => {
                            if v.is_null() {
                                continue;
                            }
                            pricing_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayModelPricingAttributes {
                    effective_source,
                    pricing_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingAttributesVisitor)
    }
}

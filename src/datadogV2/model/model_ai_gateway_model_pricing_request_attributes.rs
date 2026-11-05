// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a model pricing request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingRequestAttributes {
    /// Pricing configuration accepted when setting model pricing.
    #[serde(rename = "pricing_config")]
    pub pricing_config: crate::datadogV2::model::AiGatewayModelPricingRequestConfig,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingRequestAttributes {
    pub fn new(
        pricing_config: crate::datadogV2::model::AiGatewayModelPricingRequestConfig,
    ) -> AiGatewayModelPricingRequestAttributes {
        AiGatewayModelPricingRequestAttributes {
            pricing_config,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingRequestAttributesVisitor {
            type Value = AiGatewayModelPricingRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pricing_config: Option<
                    crate::datadogV2::model::AiGatewayModelPricingRequestConfig,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pricing_config" => {
                            pricing_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pricing_config) = pricing_config {
                                match _pricing_config {
                                    crate::datadogV2::model::AiGatewayModelPricingRequestConfig::UnparsedObject(_pricing_config) => {
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
                let pricing_config =
                    pricing_config.ok_or_else(|| M::Error::missing_field("pricing_config"))?;

                let content = AiGatewayModelPricingRequestAttributes {
                    pricing_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingRequestAttributesVisitor)
    }
}

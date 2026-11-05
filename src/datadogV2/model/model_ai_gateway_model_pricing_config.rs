// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pricing configuration for a model.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingConfig {
    /// Pricing rates for a model.
    #[serde(rename = "overrides")]
    pub overrides: Option<crate::datadogV2::model::AiGatewayModelPricingRates>,
    /// Pricing rates for a model.
    #[serde(rename = "rates")]
    pub rates: Option<crate::datadogV2::model::AiGatewayModelPricingRates>,
    /// Source information for a model's pricing.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelPricingSource,
    /// Pricing configuration version. The only supported version is `1`.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingConfig {
    pub fn new(
        source: crate::datadogV2::model::AiGatewayModelPricingSource,
        version: i64,
    ) -> AiGatewayModelPricingConfig {
        AiGatewayModelPricingConfig {
            overrides: None,
            rates: None,
            source,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn overrides(mut self, value: crate::datadogV2::model::AiGatewayModelPricingRates) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn rates(mut self, value: crate::datadogV2::model::AiGatewayModelPricingRates) -> Self {
        self.rates = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayModelPricingConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingConfigVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingConfigVisitor {
            type Value = AiGatewayModelPricingConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut overrides: Option<crate::datadogV2::model::AiGatewayModelPricingRates> =
                    None;
                let mut rates: Option<crate::datadogV2::model::AiGatewayModelPricingRates> = None;
                let mut source: Option<crate::datadogV2::model::AiGatewayModelPricingSource> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "overrides" => {
                            if v.is_null() {
                                continue;
                            }
                            overrides = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rates" => {
                            if v.is_null() {
                                continue;
                            }
                            rates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = AiGatewayModelPricingConfig {
                    overrides,
                    rates,
                    source,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingConfigVisitor)
    }
}

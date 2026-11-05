// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pricing rates for a model.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingRates {
    /// Cost per million tokens for cache reads.
    #[serde(rename = "cache_read_cost_per_mtok")]
    pub cache_read_cost_per_mtok: Option<f64>,
    /// Cost per million tokens for cache writes.
    #[serde(rename = "cache_write_cost_per_mtok")]
    pub cache_write_cost_per_mtok: Option<f64>,
    /// Cost per million tokens for non-cached input.
    #[serde(rename = "non_cached_input_cost_per_mtok")]
    pub non_cached_input_cost_per_mtok: Option<f64>,
    /// Cost per million tokens for output.
    #[serde(rename = "output_cost_per_mtok")]
    pub output_cost_per_mtok: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingRates {
    pub fn new() -> AiGatewayModelPricingRates {
        AiGatewayModelPricingRates {
            cache_read_cost_per_mtok: None,
            cache_write_cost_per_mtok: None,
            non_cached_input_cost_per_mtok: None,
            output_cost_per_mtok: None,
            _unparsed: false,
        }
    }

    pub fn cache_read_cost_per_mtok(mut self, value: f64) -> Self {
        self.cache_read_cost_per_mtok = Some(value);
        self
    }

    pub fn cache_write_cost_per_mtok(mut self, value: f64) -> Self {
        self.cache_write_cost_per_mtok = Some(value);
        self
    }

    pub fn non_cached_input_cost_per_mtok(mut self, value: f64) -> Self {
        self.non_cached_input_cost_per_mtok = Some(value);
        self
    }

    pub fn output_cost_per_mtok(mut self, value: f64) -> Self {
        self.output_cost_per_mtok = Some(value);
        self
    }
}

impl Default for AiGatewayModelPricingRates {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingRates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingRatesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingRatesVisitor {
            type Value = AiGatewayModelPricingRates;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cache_read_cost_per_mtok: Option<f64> = None;
                let mut cache_write_cost_per_mtok: Option<f64> = None;
                let mut non_cached_input_cost_per_mtok: Option<f64> = None;
                let mut output_cost_per_mtok: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cache_read_cost_per_mtok" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cache_read_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cache_write_cost_per_mtok" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cache_write_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "non_cached_input_cost_per_mtok" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            non_cached_input_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_cost_per_mtok" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            output_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AiGatewayModelPricingRates {
                    cache_read_cost_per_mtok,
                    cache_write_cost_per_mtok,
                    non_cached_input_cost_per_mtok,
                    output_cost_per_mtok,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingRatesVisitor)
    }
}

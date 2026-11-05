// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Complete set of rates required for custom pricing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingRequiredRates {
    /// Cost per million tokens for cache reads.
    #[serde(rename = "cache_read_cost_per_mtok")]
    pub cache_read_cost_per_mtok: f64,
    /// Cost per million tokens for cache writes.
    #[serde(rename = "cache_write_cost_per_mtok")]
    pub cache_write_cost_per_mtok: f64,
    /// Cost per million tokens for non-cached input.
    #[serde(rename = "non_cached_input_cost_per_mtok")]
    pub non_cached_input_cost_per_mtok: f64,
    /// Cost per million tokens for output.
    #[serde(rename = "output_cost_per_mtok")]
    pub output_cost_per_mtok: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingRequiredRates {
    pub fn new(
        cache_read_cost_per_mtok: f64,
        cache_write_cost_per_mtok: f64,
        non_cached_input_cost_per_mtok: f64,
        output_cost_per_mtok: f64,
    ) -> AiGatewayModelPricingRequiredRates {
        AiGatewayModelPricingRequiredRates {
            cache_read_cost_per_mtok,
            cache_write_cost_per_mtok,
            non_cached_input_cost_per_mtok,
            output_cost_per_mtok,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingRequiredRates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingRequiredRatesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingRequiredRatesVisitor {
            type Value = AiGatewayModelPricingRequiredRates;

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
                            cache_read_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cache_write_cost_per_mtok" => {
                            cache_write_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "non_cached_input_cost_per_mtok" => {
                            non_cached_input_cost_per_mtok =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_cost_per_mtok" => {
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
                let cache_read_cost_per_mtok = cache_read_cost_per_mtok
                    .ok_or_else(|| M::Error::missing_field("cache_read_cost_per_mtok"))?;
                let cache_write_cost_per_mtok = cache_write_cost_per_mtok
                    .ok_or_else(|| M::Error::missing_field("cache_write_cost_per_mtok"))?;
                let non_cached_input_cost_per_mtok = non_cached_input_cost_per_mtok
                    .ok_or_else(|| M::Error::missing_field("non_cached_input_cost_per_mtok"))?;
                let output_cost_per_mtok = output_cost_per_mtok
                    .ok_or_else(|| M::Error::missing_field("output_cost_per_mtok"))?;

                let content = AiGatewayModelPricingRequiredRates {
                    cache_read_cost_per_mtok,
                    cache_write_cost_per_mtok,
                    non_cached_input_cost_per_mtok,
                    output_cost_per_mtok,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingRequiredRatesVisitor)
    }
}

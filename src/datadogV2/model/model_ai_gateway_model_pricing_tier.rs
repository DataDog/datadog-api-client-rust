// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom price for requests at or above `context_min_tokens` input tokens, in `service_tier` when it is set.
/// Each tier needs a `service_tier`, a positive `context_min_tokens`, or both. Rates it omits keep the base custom rate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingTier {
    /// Minimum input tokens for the price to apply.
    #[serde(rename = "context_min_tokens")]
    pub context_min_tokens: Option<i64>,
    /// Pricing rates for a model.
    #[serde(rename = "rates")]
    pub rates: crate::datadogV2::model::AiGatewayModelPricingRates,
    /// Provider service tier the price applies to. Lowercase letters, digits, `_` and `-`.
    #[serde(rename = "service_tier")]
    pub service_tier: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelPricingTier {
    pub fn new(
        rates: crate::datadogV2::model::AiGatewayModelPricingRates,
    ) -> AiGatewayModelPricingTier {
        AiGatewayModelPricingTier {
            context_min_tokens: None,
            rates,
            service_tier: None,
            _unparsed: false,
        }
    }

    pub fn context_min_tokens(mut self, value: i64) -> Self {
        self.context_min_tokens = Some(value);
        self
    }

    pub fn service_tier(mut self, value: String) -> Self {
        self.service_tier = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelPricingTier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelPricingTierVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelPricingTierVisitor {
            type Value = AiGatewayModelPricingTier;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context_min_tokens: Option<i64> = None;
                let mut rates: Option<crate::datadogV2::model::AiGatewayModelPricingRates> = None;
                let mut service_tier: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "context_min_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            context_min_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rates" => {
                            rates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            service_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let rates = rates.ok_or_else(|| M::Error::missing_field("rates"))?;

                let content = AiGatewayModelPricingTier {
                    context_min_tokens,
                    rates,
                    service_tier,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingTierVisitor)
    }
}

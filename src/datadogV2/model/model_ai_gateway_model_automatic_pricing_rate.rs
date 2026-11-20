// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// One published price. Prices can differ by operation, mode, service tier, and input length.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelAutomaticPricingRate {
    /// Exact decimal amount per unit, zero or greater, represented as a string to preserve precision. Supports fixed decimal and exponent notation.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Token category priced, such as `input`, `output`, `cache_read`, or `cache_write`.
    #[serde(rename = "component")]
    pub component: String,
    /// Largest input length covered by this price. Omitted when there is no upper bound.
    #[serde(rename = "context_max_tokens")]
    pub context_max_tokens: Option<i64>,
    /// Input length above which this price applies. Omitted for the base tier.
    #[serde(rename = "context_min_tokens")]
    pub context_min_tokens: Option<i64>,
    /// Currency of the price. The only supported value is USD.
    #[serde(rename = "currency")]
    pub currency: String,
    /// Source mode name. `standard` is the base mode. Other values, such as `fast`, are source-defined and
/// selected by `mode_triggers`.
    #[serde(rename = "mode")]
    pub mode: String,
    /// Operation the price applies to. Omitted when it applies to every operation.
    #[serde(rename = "operation")]
    pub operation: Option<String>,
    /// Provider service tier explicitly associated with this mode by the source. Omitted when unknown.
    #[serde(rename = "service_tier")]
    pub service_tier: Option<String>,
    /// Unit of the quoted amount. The only supported value is million_tokens.
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayModelAutomaticPricingRate {
    pub fn new(amount: String, component: String, currency: String, mode: String, unit: String) -> AiGatewayModelAutomaticPricingRate {
        AiGatewayModelAutomaticPricingRate {
            amount,
            component,
            context_max_tokens: None,
            context_min_tokens: None,
            currency,
            mode,
            operation: None,
            service_tier: None,
            unit,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn context_max_tokens(mut self, value: i64) -> Self {
        self.context_max_tokens = Some(value);
        self
    }
    
    pub fn context_min_tokens(mut self, value: i64) -> Self {
        self.context_min_tokens = Some(value);
        self
    }
    
    pub fn operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }
    
    pub fn service_tier(mut self, value: String) -> Self {
        self.service_tier = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayModelAutomaticPricingRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelAutomaticPricingRateVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelAutomaticPricingRateVisitor {
            type Value = AiGatewayModelAutomaticPricingRate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut amount: Option<String> = None;
                let mut component: Option<String> = None;
                let mut context_max_tokens: Option<i64> = None;
                let mut context_min_tokens: Option<i64> = None;
                let mut currency: Option<String> = None;
                let mut mode: Option<String> = None;
                let mut operation: Option<String> = None;
                let mut service_tier: Option<String> = None;
                let mut unit: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "amount" => {
                            amount = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "component" => {
                            component = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "context_max_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            context_max_tokens = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "context_min_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            context_min_tokens = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "currency" => {
                            currency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "service_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            service_tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let amount = amount.ok_or_else(|| M::Error::missing_field("amount"))?;
                let component = component.ok_or_else(|| M::Error::missing_field("component"))?;
                let currency = currency.ok_or_else(|| M::Error::missing_field("currency"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;

                let content = AiGatewayModelAutomaticPricingRate {
                    amount,
                    component,
                    context_max_tokens,
                    context_min_tokens,
                    currency,
                    mode,
                    operation,
                    service_tier,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelAutomaticPricingRateVisitor)
    }
}

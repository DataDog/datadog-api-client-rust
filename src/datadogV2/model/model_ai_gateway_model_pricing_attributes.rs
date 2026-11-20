// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Attributes of a model pricing resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelPricingAttributes {
    /// True when models.dev pricing is authoritative for this organization and provider.
    #[serde(rename = "authoritative")]
    pub authoritative: Option<bool>,
    /// The published models.dev price. Present only when it is the effective price.
    #[serde(rename = "automatic_pricing")]
    pub automatic_pricing: Option<crate::datadogV2::model::AiGatewayModelAutomaticPricing>,
    /// True when a `BLOCK` budget covering this model rejects its requests before they reach the provider.
    #[serde(rename = "blocks_budgeted_requests")]
    pub blocks_budgeted_requests: Option<bool>,
    /// Effective provider multiplier applied to every reported rate.
    #[serde(rename = "cost_multiplier")]
    pub cost_multiplier: Option<f64>,
    /// True when the provider bills this model per minute of audio instead of per token, as with transcription
/// models. The gateway prices only per token, so it can't charge for these models, and a per-token
/// override can't express their rate.
    #[serde(rename = "duration_priced")]
    pub duration_priced: Option<bool>,
    /// Effective source of the model pricing data.
    #[serde(rename = "effective_source")]
    pub effective_source: Option<crate::datadogV2::model::AiGatewayModelPricingEffectiveSource>,
    /// Pricing configuration for a model.
    #[serde(rename = "pricing_config")]
    pub pricing_config: Option<crate::datadogV2::model::AiGatewayModelPricingConfig>,
    /// Whether the gateway has a price for the model.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::AiGatewayModelPricingStatus>,
    /// Human-readable sentence for `unpriced_reason`. Omitted when the model is priced or the code has no description yet.
    #[serde(rename = "unpriced_description")]
    pub unpriced_description: Option<String>,
    /// Stable code explaining why the model has no price, such as `automatic_price_absent`. Omitted when the model is priced.
    #[serde(rename = "unpriced_reason")]
    pub unpriced_reason: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayModelPricingAttributes {
    pub fn new() -> AiGatewayModelPricingAttributes {
        AiGatewayModelPricingAttributes {
            authoritative: None,
            automatic_pricing: None,
            blocks_budgeted_requests: None,
            cost_multiplier: None,
            duration_priced: None,
            effective_source: None,
            pricing_config: None,
            status: None,
            unpriced_description: None,
            unpriced_reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn authoritative(mut self, value: bool) -> Self {
        self.authoritative = Some(value);
        self
    }
    
    pub fn automatic_pricing(mut self, value: crate::datadogV2::model::AiGatewayModelAutomaticPricing) -> Self {
        self.automatic_pricing = Some(value);
        self
    }
    
    pub fn blocks_budgeted_requests(mut self, value: bool) -> Self {
        self.blocks_budgeted_requests = Some(value);
        self
    }
    
    pub fn cost_multiplier(mut self, value: f64) -> Self {
        self.cost_multiplier = Some(value);
        self
    }
    
    pub fn duration_priced(mut self, value: bool) -> Self {
        self.duration_priced = Some(value);
        self
    }
    
    pub fn effective_source(mut self, value: crate::datadogV2::model::AiGatewayModelPricingEffectiveSource) -> Self {
        self.effective_source = Some(value);
        self
    }
    
    pub fn pricing_config(mut self, value: crate::datadogV2::model::AiGatewayModelPricingConfig) -> Self {
        self.pricing_config = Some(value);
        self
    }
    
    pub fn status(mut self, value: crate::datadogV2::model::AiGatewayModelPricingStatus) -> Self {
        self.status = Some(value);
        self
    }
    
    pub fn unpriced_description(mut self, value: String) -> Self {
        self.unpriced_description = Some(value);
        self
    }
    
    pub fn unpriced_reason(mut self, value: String) -> Self {
        self.unpriced_reason = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
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
                let mut authoritative: Option<bool> = None;
                let mut automatic_pricing: Option<crate::datadogV2::model::AiGatewayModelAutomaticPricing> = None;
                let mut blocks_budgeted_requests: Option<bool> = None;
                let mut cost_multiplier: Option<f64> = None;
                let mut duration_priced: Option<bool> = None;
                let mut effective_source: Option<crate::datadogV2::model::AiGatewayModelPricingEffectiveSource> = None;
                let mut pricing_config: Option<crate::datadogV2::model::AiGatewayModelPricingConfig> = None;
                let mut status: Option<crate::datadogV2::model::AiGatewayModelPricingStatus> = None;
                let mut unpriced_description: Option<String> = None;
                let mut unpriced_reason: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authoritative" => {
                            if v.is_null() {
                                continue;
                            }
                            authoritative = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "automatic_pricing" => {
                            if v.is_null() {
                                continue;
                            }
                            automatic_pricing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "blocks_budgeted_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            blocks_budgeted_requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cost_multiplier" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cost_multiplier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "duration_priced" => {
                            if v.is_null() {
                                continue;
                            }
                            duration_priced = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "effective_source" => {
                            if v.is_null() {
                                continue;
                            }
                            effective_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _effective_source) = effective_source {
                                match _effective_source {
                                    crate::datadogV2::model::AiGatewayModelPricingEffectiveSource::UnparsedObject(_effective_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "pricing_config" => {
                            if v.is_null() {
                                continue;
                            }
                            pricing_config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::AiGatewayModelPricingStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unpriced_description" => {
                            if v.is_null() {
                                continue;
                            }
                            unpriced_description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "unpriced_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            unpriced_reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = AiGatewayModelPricingAttributes {
                    authoritative,
                    automatic_pricing,
                    blocks_budgeted_requests,
                    cost_multiplier,
                    duration_priced,
                    effective_source,
                    pricing_config,
                    status,
                    unpriced_description,
                    unpriced_reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelPricingAttributesVisitor)
    }
}

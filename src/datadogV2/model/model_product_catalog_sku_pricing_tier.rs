// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A usage range and the price that applies to usage falling inside it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductCatalogSKUPricingTier {
    /// The exclusive upper bound of the usage range the tier prices. `null` on the final
    /// tier, which is unbounded.
    #[serialize_always]
    #[serde(rename = "max_usage_quantity")]
    pub max_usage_quantity: Option<i64>,
    /// The inclusive lower bound of the usage range the tier prices.
    #[serde(rename = "min_usage_quantity")]
    pub min_usage_quantity: i64,
    /// The price applied to usage in the tier, as a decimal string. The number of decimal
    /// places is not normalized, so free tiers appear as either `0` or `0.00`.
    #[serde(rename = "price")]
    pub price: String,
    /// Whether the tier's price applies per unit of usage or to a block of usage.
    #[serde(rename = "pricing_unit_type")]
    pub pricing_unit_type: crate::datadogV2::model::ProductCatalogSKUPricingUnitType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductCatalogSKUPricingTier {
    pub fn new(
        max_usage_quantity: Option<i64>,
        min_usage_quantity: i64,
        price: String,
        pricing_unit_type: crate::datadogV2::model::ProductCatalogSKUPricingUnitType,
    ) -> ProductCatalogSKUPricingTier {
        ProductCatalogSKUPricingTier {
            max_usage_quantity,
            min_usage_quantity,
            price,
            pricing_unit_type,
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

impl<'de> Deserialize<'de> for ProductCatalogSKUPricingTier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductCatalogSKUPricingTierVisitor;
        impl<'a> Visitor<'a> for ProductCatalogSKUPricingTierVisitor {
            type Value = ProductCatalogSKUPricingTier;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_usage_quantity: Option<Option<i64>> = None;
                let mut min_usage_quantity: Option<i64> = None;
                let mut price: Option<String> = None;
                let mut pricing_unit_type: Option<
                    crate::datadogV2::model::ProductCatalogSKUPricingUnitType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_usage_quantity" => {
                            max_usage_quantity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_usage_quantity" => {
                            min_usage_quantity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "price" => {
                            price = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pricing_unit_type" => {
                            pricing_unit_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pricing_unit_type) = pricing_unit_type {
                                match _pricing_unit_type {
                                    crate::datadogV2::model::ProductCatalogSKUPricingUnitType::UnparsedObject(_pricing_unit_type) => {
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
                let max_usage_quantity = max_usage_quantity
                    .ok_or_else(|| M::Error::missing_field("max_usage_quantity"))?;
                let min_usage_quantity = min_usage_quantity
                    .ok_or_else(|| M::Error::missing_field("min_usage_quantity"))?;
                let price = price.ok_or_else(|| M::Error::missing_field("price"))?;
                let pricing_unit_type = pricing_unit_type
                    .ok_or_else(|| M::Error::missing_field("pricing_unit_type"))?;

                let content = ProductCatalogSKUPricingTier {
                    max_usage_quantity,
                    min_usage_quantity,
                    price,
                    pricing_unit_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductCatalogSKUPricingTierVisitor)
    }
}

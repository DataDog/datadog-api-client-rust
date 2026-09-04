// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The pricing and allotment metadata of a SKU.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductCatalogSKUDataAttributesResponse {
    /// The allotments the SKU provides to other SKUs. Every entry carries the code of this
    /// SKU as its `parent_sku_code`. Empty when the SKU provides no allotments.
    #[serde(rename = "allotments")]
    pub allotments: Vec<crate::datadogV2::model::ProductCatalogSKUAllotment>,
    /// The identifier of the billing dimension the SKU is billed on, as used by the usage
    /// metering endpoints. Several SKUs can share one billing dimension, so this value does
    /// not identify a SKU.
    #[serde(rename = "billing_dimension")]
    pub billing_dimension: String,
    /// The billable usage unit the SKU is priced per. `null` for SKUs that are not priced
    /// per unit of usage, such as those whose `pricing_type` is `percent`.
    #[serialize_always]
    #[serde(rename = "billing_units")]
    pub billing_units: Option<String>,
    /// The ISO-4217 code of the currency the prices are expressed in.
    #[serde(rename = "currency")]
    pub currency: String,
    /// The billing frequency applied to on-demand usage of the SKU by default.
    #[serde(rename = "default_on_demand_option")]
    pub default_on_demand_option: crate::datadogV2::model::ProductCatalogSKUOnDemandOption,
    /// The number of billable usage units that one unit of price covers. Divide measured
    /// usage by this value before multiplying by the price. For example, a SKU priced at `18.00` with
    /// `number_of_units_included_in_price` of `1` costs `18.00` per host, while a SKU priced
    /// at `12.00` with `number_of_units_included_in_price` of `10000` costs `12.00` per
    /// 10,000 requests. It is a scaling factor on the price, not a free allotment; included
    /// quantities are in `allotments`. The same factor applies to the price of a tier in
    /// `on_demand_tiered` whose `pricing_unit_type` is `unit`. It does not apply to a tier
    /// whose `pricing_unit_type` is `block`: that tier's `price` is charged for the whole
    /// block bounded by `min_usage_quantity` and `max_usage_quantity`, however much of the
    /// block is used. `0` for SKUs that are not priced per unit of usage, such as those
    /// whose `pricing_type` is `percent`.
    #[serde(rename = "number_of_units_included_in_price")]
    pub number_of_units_included_in_price: i64,
    /// The public list price of on-demand usage of the SKU, as a decimal string. The number
    /// of decimal places is not normalized, so values such as `0`, `0.9`, and `30000.00`
    /// all occur. `null` when the SKU is priced with tiers, in which case the prices are in
    /// `on_demand_tiered`.
    #[serialize_always]
    #[serde(rename = "on_demand_list_price")]
    pub on_demand_list_price: Option<String>,
    /// The tiered pricing applied to on-demand usage of the SKU. `null` when the SKU is priced
    /// with a single list price instead.
    #[serialize_always]
    #[serde(rename = "on_demand_tiered")]
    pub on_demand_tiered: Option<crate::datadogV2::model::ProductCatalogSKUTieredPricing>,
    /// How the SKU is priced. `usage` prices each billable usage unit, and `percent` prices a
    /// percentage; percent-priced SKUs have no `billing_units`.
    #[serde(rename = "pricing_type")]
    pub pricing_type: crate::datadogV2::model::ProductCatalogSKUPricingType,
    /// The human-readable name of the SKU.
    #[serde(rename = "sku_name")]
    pub sku_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductCatalogSKUDataAttributesResponse {
    pub fn new(
        allotments: Vec<crate::datadogV2::model::ProductCatalogSKUAllotment>,
        billing_dimension: String,
        billing_units: Option<String>,
        currency: String,
        default_on_demand_option: crate::datadogV2::model::ProductCatalogSKUOnDemandOption,
        number_of_units_included_in_price: i64,
        on_demand_list_price: Option<String>,
        on_demand_tiered: Option<crate::datadogV2::model::ProductCatalogSKUTieredPricing>,
        pricing_type: crate::datadogV2::model::ProductCatalogSKUPricingType,
        sku_name: String,
    ) -> ProductCatalogSKUDataAttributesResponse {
        ProductCatalogSKUDataAttributesResponse {
            allotments,
            billing_dimension,
            billing_units,
            currency,
            default_on_demand_option,
            number_of_units_included_in_price,
            on_demand_list_price,
            on_demand_tiered,
            pricing_type,
            sku_name,
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

impl<'de> Deserialize<'de> for ProductCatalogSKUDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductCatalogSKUDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for ProductCatalogSKUDataAttributesResponseVisitor {
            type Value = ProductCatalogSKUDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allotments: Option<
                    Vec<crate::datadogV2::model::ProductCatalogSKUAllotment>,
                > = None;
                let mut billing_dimension: Option<String> = None;
                let mut billing_units: Option<Option<String>> = None;
                let mut currency: Option<String> = None;
                let mut default_on_demand_option: Option<
                    crate::datadogV2::model::ProductCatalogSKUOnDemandOption,
                > = None;
                let mut number_of_units_included_in_price: Option<i64> = None;
                let mut on_demand_list_price: Option<Option<String>> = None;
                let mut on_demand_tiered: Option<
                    Option<crate::datadogV2::model::ProductCatalogSKUTieredPricing>,
                > = None;
                let mut pricing_type: Option<
                    crate::datadogV2::model::ProductCatalogSKUPricingType,
                > = None;
                let mut sku_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allotments" => {
                            allotments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billing_dimension" => {
                            billing_dimension =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "billing_units" => {
                            billing_units =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "currency" => {
                            currency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_on_demand_option" => {
                            default_on_demand_option =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _default_on_demand_option) = default_on_demand_option {
                                match _default_on_demand_option {
                                    crate::datadogV2::model::ProductCatalogSKUOnDemandOption::UnparsedObject(_default_on_demand_option) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "number_of_units_included_in_price" => {
                            number_of_units_included_in_price =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_demand_list_price" => {
                            on_demand_list_price =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_demand_tiered" => {
                            on_demand_tiered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pricing_type" => {
                            pricing_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _pricing_type) = pricing_type {
                                match _pricing_type {
                                    crate::datadogV2::model::ProductCatalogSKUPricingType::UnparsedObject(_pricing_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sku_name" => {
                            sku_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allotments = allotments.ok_or_else(|| M::Error::missing_field("allotments"))?;
                let billing_dimension = billing_dimension
                    .ok_or_else(|| M::Error::missing_field("billing_dimension"))?;
                let billing_units =
                    billing_units.ok_or_else(|| M::Error::missing_field("billing_units"))?;
                let currency = currency.ok_or_else(|| M::Error::missing_field("currency"))?;
                let default_on_demand_option = default_on_demand_option
                    .ok_or_else(|| M::Error::missing_field("default_on_demand_option"))?;
                let number_of_units_included_in_price = number_of_units_included_in_price
                    .ok_or_else(|| M::Error::missing_field("number_of_units_included_in_price"))?;
                let on_demand_list_price = on_demand_list_price
                    .ok_or_else(|| M::Error::missing_field("on_demand_list_price"))?;
                let on_demand_tiered =
                    on_demand_tiered.ok_or_else(|| M::Error::missing_field("on_demand_tiered"))?;
                let pricing_type =
                    pricing_type.ok_or_else(|| M::Error::missing_field("pricing_type"))?;
                let sku_name = sku_name.ok_or_else(|| M::Error::missing_field("sku_name"))?;

                let content = ProductCatalogSKUDataAttributesResponse {
                    allotments,
                    billing_dimension,
                    billing_units,
                    currency,
                    default_on_demand_option,
                    number_of_units_included_in_price,
                    on_demand_list_price,
                    on_demand_tiered,
                    pricing_type,
                    sku_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductCatalogSKUDataAttributesResponseVisitor)
    }
}

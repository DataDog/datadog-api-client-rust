// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Line item details from a Custom Costs file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostsFileLineItem {
    /// Total cost in the cost file.
    #[serde(rename = "BilledCost")]
    pub billed_cost: Option<f64>,
    /// Currency used in the Custom Costs file.
    #[serde(rename = "BillingCurrency")]
    pub billing_currency: Option<String>,
    /// Description for the line item cost.
    #[serde(rename = "ChargeDescription")]
    pub charge_description: Option<String>,
    /// End date of the usage charge.
    #[serde(rename = "ChargePeriodEnd")]
    pub charge_period_end: Option<String>,
    /// Start date of the usage charge.
    #[serde(rename = "ChargePeriodStart")]
    pub charge_period_start: Option<String>,
    /// Name of the provider for the line item.
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
    /// Additional tags for the line item.
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostsFileLineItem {
    pub fn new() -> CustomCostsFileLineItem {
        CustomCostsFileLineItem {
            billed_cost: None,
            billing_currency: None,
            charge_description: None,
            charge_period_end: None,
            charge_period_start: None,
            provider_name: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn billed_cost(mut self, value: f64) -> Self {
        self.billed_cost = Some(value);
        self
    }

    pub fn billing_currency(mut self, value: String) -> Self {
        self.billing_currency = Some(value);
        self
    }

    pub fn charge_description(mut self, value: String) -> Self {
        self.charge_description = Some(value);
        self
    }

    pub fn charge_period_end(mut self, value: String) -> Self {
        self.charge_period_end = Some(value);
        self
    }

    pub fn charge_period_start(mut self, value: String) -> Self {
        self.charge_period_start = Some(value);
        self
    }

    pub fn provider_name(mut self, value: String) -> Self {
        self.provider_name = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
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

impl Default for CustomCostsFileLineItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostsFileLineItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostsFileLineItemVisitor;
        impl<'a> Visitor<'a> for CustomCostsFileLineItemVisitor {
            type Value = CustomCostsFileLineItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billed_cost: Option<f64> = None;
                let mut billing_currency: Option<String> = None;
                let mut charge_description: Option<String> = None;
                let mut charge_period_end: Option<String> = None;
                let mut charge_period_start: Option<String> = None;
                let mut provider_name: Option<String> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "BilledCost" => {
                            if v.is_null() {
                                continue;
                            }
                            billed_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "BillingCurrency" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_currency =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ChargeDescription" => {
                            if v.is_null() {
                                continue;
                            }
                            charge_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ChargePeriodEnd" => {
                            if v.is_null() {
                                continue;
                            }
                            charge_period_end =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ChargePeriodStart" => {
                            if v.is_null() {
                                continue;
                            }
                            charge_period_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ProviderName" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "Tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomCostsFileLineItem {
                    billed_cost,
                    billing_currency,
                    charge_description,
                    charge_period_end,
                    charge_period_start,
                    provider_name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostsFileLineItemVisitor)
    }
}

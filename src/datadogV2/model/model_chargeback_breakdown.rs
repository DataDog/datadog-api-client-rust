// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Charges breakdown.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChargebackBreakdown {
    /// The type of charge for a particular product.
    #[serde(rename = "charge_type")]
    pub charge_type: Option<String>,
    /// The cost for a particular product and charge type during a given month.
    #[serde(rename = "cost")]
    pub cost: Option<f64>,
    /// The product for which cost is being reported.
    #[serde(rename = "product_name")]
    pub product_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChargebackBreakdown {
    pub fn new() -> ChargebackBreakdown {
        ChargebackBreakdown {
            charge_type: None,
            cost: None,
            product_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn charge_type(mut self, value: String) -> Self {
        self.charge_type = Some(value);
        self
    }

    pub fn cost(mut self, value: f64) -> Self {
        self.cost = Some(value);
        self
    }

    pub fn product_name(mut self, value: String) -> Self {
        self.product_name = Some(value);
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

impl Default for ChargebackBreakdown {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChargebackBreakdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChargebackBreakdownVisitor;
        impl<'a> Visitor<'a> for ChargebackBreakdownVisitor {
            type Value = ChargebackBreakdown;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut charge_type: Option<String> = None;
                let mut cost: Option<f64> = None;
                let mut product_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "charge_type" => {
                            if v.is_null() {
                                continue;
                            }
                            charge_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cost" => {
                            if v.is_null() {
                                continue;
                            }
                            cost = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_name" => {
                            if v.is_null() {
                                continue;
                            }
                            product_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChargebackBreakdown {
                    charge_type,
                    cost,
                    product_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChargebackBreakdownVisitor)
    }
}

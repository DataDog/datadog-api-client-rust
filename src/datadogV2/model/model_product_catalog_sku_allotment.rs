// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A quantity of one SKU that is included with, and consumed before, the billable usage of
/// another SKU.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductCatalogSKUAllotment {
    /// The code of the SKU that receives the allotment.
    #[serde(rename = "child_sku_code")]
    pub child_sku_code: String,
    /// The quantity allotted per hour. Fractional for some allotments, and equal to
    /// `monthly_quantity` for others, depending on how the child SKU meters usage.
    #[serde(rename = "hourly_quantity")]
    pub hourly_quantity: f64,
    /// The quantity allotted per month.
    #[serde(rename = "monthly_quantity")]
    pub monthly_quantity: i64,
    /// The code of the SKU that provides the allotment. Always the code of the SKU the
    /// allotment is returned under.
    #[serde(rename = "parent_sku_code")]
    pub parent_sku_code: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductCatalogSKUAllotment {
    pub fn new(
        child_sku_code: String,
        hourly_quantity: f64,
        monthly_quantity: i64,
        parent_sku_code: String,
    ) -> ProductCatalogSKUAllotment {
        ProductCatalogSKUAllotment {
            child_sku_code,
            hourly_quantity,
            monthly_quantity,
            parent_sku_code,
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

impl<'de> Deserialize<'de> for ProductCatalogSKUAllotment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductCatalogSKUAllotmentVisitor;
        impl<'a> Visitor<'a> for ProductCatalogSKUAllotmentVisitor {
            type Value = ProductCatalogSKUAllotment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child_sku_code: Option<String> = None;
                let mut hourly_quantity: Option<f64> = None;
                let mut monthly_quantity: Option<i64> = None;
                let mut parent_sku_code: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "child_sku_code" => {
                            child_sku_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hourly_quantity" => {
                            hourly_quantity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monthly_quantity" => {
                            monthly_quantity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_sku_code" => {
                            parent_sku_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let child_sku_code =
                    child_sku_code.ok_or_else(|| M::Error::missing_field("child_sku_code"))?;
                let hourly_quantity =
                    hourly_quantity.ok_or_else(|| M::Error::missing_field("hourly_quantity"))?;
                let monthly_quantity =
                    monthly_quantity.ok_or_else(|| M::Error::missing_field("monthly_quantity"))?;
                let parent_sku_code =
                    parent_sku_code.ok_or_else(|| M::Error::missing_field("parent_sku_code"))?;

                let content = ProductCatalogSKUAllotment {
                    child_sku_code,
                    hourly_quantity,
                    monthly_quantity,
                    parent_sku_code,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductCatalogSKUAllotmentVisitor)
    }
}

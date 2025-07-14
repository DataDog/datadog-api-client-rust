// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Product-specific filters for the dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FiltersPerProduct {
    /// Defines the list of tag-based filters used to restrict access to telemetry data for a specific product.
    /// These filters act as access control rules. Each filter must follow the tag query syntax used by
    /// Datadog (such as `@tag.key:value`), and only one tag or attribute may be used to define the access strategy
    /// per telemetry type.
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// Name of the product the dataset is for. Possible values are 'apm', 'rum', 'synthetics',
    /// 'metrics', 'logs', 'sd_repoinfo', 'error_tracking', 'cloud_cost', and 'ml_obs'.
    #[serde(rename = "product")]
    pub product: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FiltersPerProduct {
    pub fn new(filters: Vec<String>, product: String) -> FiltersPerProduct {
        FiltersPerProduct {
            filters,
            product,
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

impl<'de> Deserialize<'de> for FiltersPerProduct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FiltersPerProductVisitor;
        impl<'a> Visitor<'a> for FiltersPerProductVisitor {
            type Value = FiltersPerProduct;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filters: Option<Vec<String>> = None;
                let mut product: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;

                let content = FiltersPerProduct {
                    filters,
                    product,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FiltersPerProductVisitor)
    }
}

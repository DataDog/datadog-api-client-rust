// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dataset metadata and configurations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetAttributesRequest {
    /// Name of the dataset.
    #[serde(rename = "name")]
    pub name: String,
    /// List of access principals, formatted as `principal_type:id`. Principal can be 'team' or 'role'.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// List of product-specific filters.
    #[serde(rename = "product_filters")]
    pub product_filters: Vec<crate::datadogV2::model::FiltersPerProduct>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetAttributesRequest {
    pub fn new(
        name: String,
        principals: Vec<String>,
        product_filters: Vec<crate::datadogV2::model::FiltersPerProduct>,
    ) -> DatasetAttributesRequest {
        DatasetAttributesRequest {
            name,
            principals,
            product_filters,
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

impl<'de> Deserialize<'de> for DatasetAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetAttributesRequestVisitor;
        impl<'a> Visitor<'a> for DatasetAttributesRequestVisitor {
            type Value = DatasetAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut principals: Option<Vec<String>> = None;
                let mut product_filters: Option<Vec<crate::datadogV2::model::FiltersPerProduct>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "principals" => {
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_filters" => {
                            product_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let principals = principals.ok_or_else(|| M::Error::missing_field("principals"))?;
                let product_filters =
                    product_filters.ok_or_else(|| M::Error::missing_field("product_filters"))?;

                let content = DatasetAttributesRequest {
                    name,
                    principals,
                    product_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetAttributesRequestVisitor)
    }
}

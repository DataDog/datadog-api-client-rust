// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dataset metadata and configuration(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetAttributesResponse {
    /// Timestamp when the dataset was created.
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Unique ID of the user who created the dataset.
    #[serde(rename = "created_by")]
    pub created_by: Option<uuid::Uuid>,
    /// Name of the dataset.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of access principals, formatted as `principal_type:id`. Principal can be 'team' or 'role'.
    #[serde(rename = "principals")]
    pub principals: Option<Vec<String>>,
    /// List of product-specific filters.
    #[serde(rename = "product_filters")]
    pub product_filters: Option<Vec<crate::datadogV2::model::FiltersPerProduct>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetAttributesResponse {
    pub fn new() -> DatasetAttributesResponse {
        DatasetAttributesResponse {
            created_at: None,
            created_by: None,
            name: None,
            principals: None,
            product_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: uuid::Uuid) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn principals(mut self, value: Vec<String>) -> Self {
        self.principals = Some(value);
        self
    }

    pub fn product_filters(
        mut self,
        value: Vec<crate::datadogV2::model::FiltersPerProduct>,
    ) -> Self {
        self.product_filters = Some(value);
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

impl Default for DatasetAttributesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatasetAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetAttributesResponseVisitor;
        impl<'a> Visitor<'a> for DatasetAttributesResponseVisitor {
            type Value = DatasetAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "principals" => {
                            if v.is_null() {
                                continue;
                            }
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_filters" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = DatasetAttributesResponse {
                    created_at,
                    created_by,
                    name,
                    principals,
                    product_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetAttributesResponseVisitor)
    }
}

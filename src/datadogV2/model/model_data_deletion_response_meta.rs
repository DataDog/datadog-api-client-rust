// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata of the data deletion response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataDeletionResponseMeta {
    /// The total deletion requests created by product.
    #[serde(rename = "count_product")]
    pub count_product: Option<std::collections::BTreeMap<String, i64>>,
    /// The total deletion requests created by status.
    #[serde(rename = "count_status")]
    pub count_status: Option<std::collections::BTreeMap<String, i64>>,
    /// The next page when searching deletion requests created in the current organization.
    #[serde(rename = "next_page")]
    pub next_page: Option<String>,
    /// The product of the deletion request.
    #[serde(rename = "product")]
    pub product: Option<String>,
    /// The status of the executed request.
    #[serde(rename = "request_status")]
    pub request_status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataDeletionResponseMeta {
    pub fn new() -> DataDeletionResponseMeta {
        DataDeletionResponseMeta {
            count_product: None,
            count_status: None,
            next_page: None,
            product: None,
            request_status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn count_product(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.count_product = Some(value);
        self
    }

    pub fn count_status(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.count_status = Some(value);
        self
    }

    pub fn next_page(mut self, value: String) -> Self {
        self.next_page = Some(value);
        self
    }

    pub fn product(mut self, value: String) -> Self {
        self.product = Some(value);
        self
    }

    pub fn request_status(mut self, value: String) -> Self {
        self.request_status = Some(value);
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

impl Default for DataDeletionResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DataDeletionResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataDeletionResponseMetaVisitor;
        impl<'a> Visitor<'a> for DataDeletionResponseMetaVisitor {
            type Value = DataDeletionResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count_product: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut count_status: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut next_page: Option<String> = None;
                let mut product: Option<String> = None;
                let mut request_status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count_product" => {
                            if v.is_null() {
                                continue;
                            }
                            count_product =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "count_status" => {
                            if v.is_null() {
                                continue;
                            }
                            count_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_page" => {
                            if v.is_null() {
                                continue;
                            }
                            next_page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            if v.is_null() {
                                continue;
                            }
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_status" => {
                            if v.is_null() {
                                continue;
                            }
                            request_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DataDeletionResponseMeta {
                    count_product,
                    count_status,
                    next_page,
                    product,
                    request_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataDeletionResponseMetaVisitor)
    }
}

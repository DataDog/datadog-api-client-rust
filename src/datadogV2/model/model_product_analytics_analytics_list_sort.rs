// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The sort applied to the returned event rows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAnalyticsListSort {
    /// Name of the facet to sort the rows by.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// The direction rows are sorted in.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::ProductAnalyticsAnalyticsListSortOrder>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAnalyticsListSort {
    pub fn new() -> ProductAnalyticsAnalyticsListSort {
        ProductAnalyticsAnalyticsListSort {
            facet: None,
            order: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }

    pub fn order(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsAnalyticsListSortOrder,
    ) -> Self {
        self.order = Some(value);
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

impl Default for ProductAnalyticsAnalyticsListSort {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsAnalyticsListSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAnalyticsListSortVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAnalyticsListSortVisitor {
            type Value = ProductAnalyticsAnalyticsListSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut order: Option<
                    crate::datadogV2::model::ProductAnalyticsAnalyticsListSortOrder,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            if v.is_null() {
                                continue;
                            }
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV2::model::ProductAnalyticsAnalyticsListSortOrder::UnparsedObject(_order) => {
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

                let content = ProductAnalyticsAnalyticsListSort {
                    facet,
                    order,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAnalyticsListSortVisitor)
    }
}

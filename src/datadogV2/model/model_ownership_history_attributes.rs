// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an ownership history response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipHistoryAttributes {
    /// The list of history entries returned for this page.
    #[serde(rename = "items")]
    pub items: Vec<crate::datadogV2::model::OwnershipHistoryItem>,
    /// Cursor-based pagination metadata for the history response.
    #[serde(rename = "pagination")]
    pub pagination: crate::datadogV2::model::OwnershipHistoryPagination,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipHistoryAttributes {
    pub fn new(
        items: Vec<crate::datadogV2::model::OwnershipHistoryItem>,
        pagination: crate::datadogV2::model::OwnershipHistoryPagination,
    ) -> OwnershipHistoryAttributes {
        OwnershipHistoryAttributes {
            items,
            pagination,
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

impl<'de> Deserialize<'de> for OwnershipHistoryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipHistoryAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipHistoryAttributesVisitor {
            type Value = OwnershipHistoryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut items: Option<Vec<crate::datadogV2::model::OwnershipHistoryItem>> = None;
                let mut pagination: Option<crate::datadogV2::model::OwnershipHistoryPagination> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "items" => {
                            items = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pagination" => {
                            pagination = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let items = items.ok_or_else(|| M::Error::missing_field("items"))?;
                let pagination = pagination.ok_or_else(|| M::Error::missing_field("pagination"))?;

                let content = OwnershipHistoryAttributes {
                    items,
                    pagination,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipHistoryAttributesVisitor)
    }
}

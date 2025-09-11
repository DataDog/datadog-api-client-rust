// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for bulk inserting multiple items into a datastore.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BulkPutAppsDatastoreItemsRequestDataAttributes {
    /// How to handle conflicts when inserting items that already exist in the datastore.
    #[serde(rename = "conflict_mode")]
    pub conflict_mode: Option<crate::datadogV2::model::DatastoreItemConflictMode>,
    /// An array of items to add to the datastore, where each item is a set of key-value pairs representing the item's data. Up to 100 items can be updated in a single request.
    #[serde(rename = "values")]
    pub values: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BulkPutAppsDatastoreItemsRequestDataAttributes {
    pub fn new(
        values: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> BulkPutAppsDatastoreItemsRequestDataAttributes {
        BulkPutAppsDatastoreItemsRequestDataAttributes {
            conflict_mode: None,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conflict_mode(
        mut self,
        value: crate::datadogV2::model::DatastoreItemConflictMode,
    ) -> Self {
        self.conflict_mode = Some(value);
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

impl<'de> Deserialize<'de> for BulkPutAppsDatastoreItemsRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BulkPutAppsDatastoreItemsRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for BulkPutAppsDatastoreItemsRequestDataAttributesVisitor {
            type Value = BulkPutAppsDatastoreItemsRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conflict_mode: Option<crate::datadogV2::model::DatastoreItemConflictMode> =
                    None;
                let mut values: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conflict_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            conflict_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conflict_mode) = conflict_mode {
                                match _conflict_mode {
                                    crate::datadogV2::model::DatastoreItemConflictMode::UnparsedObject(_conflict_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = BulkPutAppsDatastoreItemsRequestDataAttributes {
                    conflict_mode,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BulkPutAppsDatastoreItemsRequestDataAttributesVisitor)
    }
}

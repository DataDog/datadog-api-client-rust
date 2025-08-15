// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ItemApiPayloadDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemApiPayloadDataAttributes {
    /// The `attributes` `created_at`.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The `attributes` `modified_at`.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The `attributes` `org_id`.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// The `attributes` `primary_column_name`.
    #[serde(rename = "primary_column_name")]
    pub primary_column_name: Option<String>,
    /// The `attributes` `signature`.
    #[serde(rename = "signature")]
    pub signature: Option<String>,
    /// The `attributes` `store_id`.
    #[serde(rename = "store_id")]
    pub store_id: Option<String>,
    /// The `attributes` `value`.
    #[serde(rename = "value")]
    pub value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ItemApiPayloadDataAttributes {
    pub fn new() -> ItemApiPayloadDataAttributes {
        ItemApiPayloadDataAttributes {
            created_at: None,
            modified_at: None,
            org_id: None,
            primary_column_name: None,
            signature: None,
            store_id: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn primary_column_name(mut self, value: String) -> Self {
        self.primary_column_name = Some(value);
        self
    }

    pub fn signature(mut self, value: String) -> Self {
        self.signature = Some(value);
        self
    }

    pub fn store_id(mut self, value: String) -> Self {
        self.store_id = Some(value);
        self
    }

    pub fn value(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.value = Some(value);
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

impl Default for ItemApiPayloadDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ItemApiPayloadDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemApiPayloadDataAttributesVisitor;
        impl<'a> Visitor<'a> for ItemApiPayloadDataAttributesVisitor {
            type Value = ItemApiPayloadDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_id: Option<i64> = None;
                let mut primary_column_name: Option<String> = None;
                let mut signature: Option<String> = None;
                let mut store_id: Option<String> = None;
                let mut value: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_column_name" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signature" => {
                            if v.is_null() {
                                continue;
                            }
                            signature = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "store_id" => {
                            if v.is_null() {
                                continue;
                            }
                            store_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ItemApiPayloadDataAttributes {
                    created_at,
                    modified_at,
                    org_id,
                    primary_column_name,
                    signature,
                    store_id,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ItemApiPayloadDataAttributesVisitor)
    }
}

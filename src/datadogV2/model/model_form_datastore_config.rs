// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for the form's associated datastore.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormDatastoreConfig {
    /// The unique identifier of the datastore.
    #[serde(rename = "datastore_id")]
    pub datastore_id: uuid::Uuid,
    /// The name of the primary key column.
    #[serde(rename = "primary_column_name")]
    pub primary_column_name: String,
    /// The strategy used for generating primary keys.
    #[serde(rename = "primary_key_generation_strategy")]
    pub primary_key_generation_strategy: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormDatastoreConfig {
    pub fn new(
        datastore_id: uuid::Uuid,
        primary_column_name: String,
        primary_key_generation_strategy: String,
    ) -> FormDatastoreConfig {
        FormDatastoreConfig {
            datastore_id,
            primary_column_name,
            primary_key_generation_strategy,
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

impl<'de> Deserialize<'de> for FormDatastoreConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormDatastoreConfigVisitor;
        impl<'a> Visitor<'a> for FormDatastoreConfigVisitor {
            type Value = FormDatastoreConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut datastore_id: Option<uuid::Uuid> = None;
                let mut primary_column_name: Option<String> = None;
                let mut primary_key_generation_strategy: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "datastore_id" => {
                            datastore_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_column_name" => {
                            primary_column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_key_generation_strategy" => {
                            primary_key_generation_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let datastore_id =
                    datastore_id.ok_or_else(|| M::Error::missing_field("datastore_id"))?;
                let primary_column_name = primary_column_name
                    .ok_or_else(|| M::Error::missing_field("primary_column_name"))?;
                let primary_key_generation_strategy = primary_key_generation_strategy
                    .ok_or_else(|| M::Error::missing_field("primary_key_generation_strategy"))?;

                let content = FormDatastoreConfig {
                    datastore_id,
                    primary_column_name,
                    primary_key_generation_strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormDatastoreConfigVisitor)
    }
}

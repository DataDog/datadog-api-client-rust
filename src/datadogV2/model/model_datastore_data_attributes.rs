// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Detailed information about a datastore.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatastoreDataAttributes {
    /// Timestamp when the datastore was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The numeric ID of the user who created the datastore.
    #[serde(rename = "creator_user_id")]
    pub creator_user_id: Option<i64>,
    /// The UUID of the user who created the datastore.
    #[serde(rename = "creator_user_uuid")]
    pub creator_user_uuid: Option<String>,
    /// A human-readable description about the datastore.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Timestamp when the datastore was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The display name of the datastore.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The ID of the organization that owns this datastore.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// The name of the primary key column for this datastore. Primary column names:
    ///   - Must abide by both [PostgreSQL naming conventions](<https://www.postgresql.org/docs/7.0/syntax525.htm>)
    ///   - Cannot exceed 63 characters
    #[serde(rename = "primary_column_name")]
    pub primary_column_name: Option<String>,
    /// Can be set to `uuid` to automatically generate primary keys when new items are added. Default value is `none`, which requires you to supply a primary key for each new item.
    #[serde(rename = "primary_key_generation_strategy")]
    pub primary_key_generation_strategy:
        Option<crate::datadogV2::model::DatastorePrimaryKeyGenerationStrategy>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatastoreDataAttributes {
    pub fn new() -> DatastoreDataAttributes {
        DatastoreDataAttributes {
            created_at: None,
            creator_user_id: None,
            creator_user_uuid: None,
            description: None,
            modified_at: None,
            name: None,
            org_id: None,
            primary_column_name: None,
            primary_key_generation_strategy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_user_id(mut self, value: i64) -> Self {
        self.creator_user_id = Some(value);
        self
    }

    pub fn creator_user_uuid(mut self, value: String) -> Self {
        self.creator_user_uuid = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

    pub fn primary_key_generation_strategy(
        mut self,
        value: crate::datadogV2::model::DatastorePrimaryKeyGenerationStrategy,
    ) -> Self {
        self.primary_key_generation_strategy = Some(value);
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

impl Default for DatastoreDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatastoreDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatastoreDataAttributesVisitor;
        impl<'a> Visitor<'a> for DatastoreDataAttributesVisitor {
            type Value = DatastoreDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creator_user_id: Option<i64> = None;
                let mut creator_user_uuid: Option<String> = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut primary_column_name: Option<String> = None;
                let mut primary_key_generation_strategy: Option<
                    crate::datadogV2::model::DatastorePrimaryKeyGenerationStrategy,
                > = None;
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
                        "creator_user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            creator_user_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator_user_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            creator_user_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "primary_key_generation_strategy" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_key_generation_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _primary_key_generation_strategy) =
                                primary_key_generation_strategy
                            {
                                match _primary_key_generation_strategy {
                                    crate::datadogV2::model::DatastorePrimaryKeyGenerationStrategy::UnparsedObject(_primary_key_generation_strategy) => {
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

                let content = DatastoreDataAttributes {
                    created_at,
                    creator_user_id,
                    creator_user_uuid,
                    description,
                    modified_at,
                    name,
                    org_id,
                    primary_column_name,
                    primary_key_generation_strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatastoreDataAttributesVisitor)
    }
}

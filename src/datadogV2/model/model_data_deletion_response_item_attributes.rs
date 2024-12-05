// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Deletion attribute for data deletion response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataDeletionResponseItemAttributes {
    /// Creation time of the deletion request.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// User who created the deletion request.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_time")]
    pub from_time: i64,
    /// List of indexes for the search. If not provided, the search is performed in all indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Whether the deletion request is fully created or not. It can take several minutes to fully create a deletion request depending on the target query and timeframe.
    #[serde(rename = "is_created")]
    pub is_created: bool,
    /// Organization ID.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Product name.
    #[serde(rename = "product")]
    pub product: String,
    /// Query for creating a data deletion request.
    #[serde(rename = "query")]
    pub query: String,
    /// Starting time of the process to delete the requested data.
    #[serde(rename = "starting_at")]
    pub starting_at: String,
    /// Status of the deletion request.
    #[serde(rename = "status")]
    pub status: String,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_time")]
    pub to_time: i64,
    /// Total number of elements to be deleted. Only the data accessible to the current user that matches the query and timeframe provided will be deleted.
    #[serde(rename = "total_unrestricted")]
    pub total_unrestricted: i64,
    /// Update time of the deletion request.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataDeletionResponseItemAttributes {
    pub fn new(
        created_at: String,
        created_by: String,
        from_time: i64,
        is_created: bool,
        org_id: i64,
        product: String,
        query: String,
        starting_at: String,
        status: String,
        to_time: i64,
        total_unrestricted: i64,
        updated_at: String,
    ) -> DataDeletionResponseItemAttributes {
        DataDeletionResponseItemAttributes {
            created_at,
            created_by,
            from_time,
            indexes: None,
            is_created,
            org_id,
            product,
            query,
            starting_at,
            status,
            to_time,
            total_unrestricted,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
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

impl<'de> Deserialize<'de> for DataDeletionResponseItemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataDeletionResponseItemAttributesVisitor;
        impl<'a> Visitor<'a> for DataDeletionResponseItemAttributesVisitor {
            type Value = DataDeletionResponseItemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut created_by: Option<String> = None;
                let mut from_time: Option<i64> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut is_created: Option<bool> = None;
                let mut org_id: Option<i64> = None;
                let mut product: Option<String> = None;
                let mut query: Option<String> = None;
                let mut starting_at: Option<String> = None;
                let mut status: Option<String> = None;
                let mut to_time: Option<i64> = None;
                let mut total_unrestricted: Option<i64> = None;
                let mut updated_at: Option<String> = None;
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
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from_time" => {
                            from_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_created" => {
                            is_created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "starting_at" => {
                            starting_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_time" => {
                            to_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_unrestricted" => {
                            total_unrestricted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let from_time = from_time.ok_or_else(|| M::Error::missing_field("from_time"))?;
                let is_created = is_created.ok_or_else(|| M::Error::missing_field("is_created"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let starting_at =
                    starting_at.ok_or_else(|| M::Error::missing_field("starting_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let to_time = to_time.ok_or_else(|| M::Error::missing_field("to_time"))?;
                let total_unrestricted = total_unrestricted
                    .ok_or_else(|| M::Error::missing_field("total_unrestricted"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = DataDeletionResponseItemAttributes {
                    created_at,
                    created_by,
                    from_time,
                    indexes,
                    is_created,
                    org_id,
                    product,
                    query,
                    starting_at,
                    status,
                    to_time,
                    total_unrestricted,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataDeletionResponseItemAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for batch-updating records in an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetBatchUpdateDataAttributesRequest {
    /// Whether to create a new dataset version when applying the batch update. Defaults to `true`.
    #[serde(rename = "create_new_version")]
    pub create_new_version: Option<bool>,
    /// Record IDs to delete.
    #[serde(rename = "delete_records")]
    pub delete_records: Option<Vec<String>>,
    /// Records to insert.
    #[serde(rename = "insert_records")]
    pub insert_records: Option<Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateInsertRecord>>,
    /// List of tag strings.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Records to update by ID.
    #[serde(rename = "update_records")]
    pub update_records: Option<Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateUpdateRecord>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetBatchUpdateDataAttributesRequest {
    pub fn new() -> LLMObsDatasetBatchUpdateDataAttributesRequest {
        LLMObsDatasetBatchUpdateDataAttributesRequest {
            create_new_version: None,
            delete_records: None,
            insert_records: None,
            tags: None,
            update_records: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn create_new_version(mut self, value: bool) -> Self {
        self.create_new_version = Some(value);
        self
    }

    pub fn delete_records(mut self, value: Vec<String>) -> Self {
        self.delete_records = Some(value);
        self
    }

    pub fn insert_records(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateInsertRecord>,
    ) -> Self {
        self.insert_records = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn update_records(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateUpdateRecord>,
    ) -> Self {
        self.update_records = Some(value);
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

impl Default for LLMObsDatasetBatchUpdateDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsDatasetBatchUpdateDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetBatchUpdateDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetBatchUpdateDataAttributesRequestVisitor {
            type Value = LLMObsDatasetBatchUpdateDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut create_new_version: Option<bool> = None;
                let mut delete_records: Option<Vec<String>> = None;
                let mut insert_records: Option<
                    Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateInsertRecord>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut update_records: Option<
                    Vec<crate::datadogV2::model::LLMObsDatasetBatchUpdateUpdateRecord>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "create_new_version" => {
                            if v.is_null() {
                                continue;
                            }
                            create_new_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "delete_records" => {
                            if v.is_null() {
                                continue;
                            }
                            delete_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "insert_records" => {
                            if v.is_null() {
                                continue;
                            }
                            insert_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "update_records" => {
                            if v.is_null() {
                                continue;
                            }
                            update_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsDatasetBatchUpdateDataAttributesRequest {
                    create_new_version,
                    delete_records,
                    insert_records,
                    tags,
                    update_records,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetBatchUpdateDataAttributesRequestVisitor)
    }
}

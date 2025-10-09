// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// File metadata for reference tables created by cloud storage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributesFileMetadataCloudStorage {
    /// The definition of `TableResultV2DataAttributesFileMetadataOneOfAccessDetails` object.
    #[serde(rename = "access_details")]
    pub access_details:
        Option<crate::datadogV2::model::TableResultV2DataAttributesFileMetadataOneOfAccessDetails>,
    /// The error message returned from the sync.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// The number of rows that failed to sync.
    #[serde(rename = "error_row_count")]
    pub error_row_count: Option<i64>,
    /// The type of error that occurred during file processing. This field provides high-level error categories for easier troubleshooting and is only present when there are errors.
    #[serde(rename = "error_type")]
    pub error_type: Option<
        crate::datadogV2::model::TableResultV2DataAttributesFileMetadataCloudStorageErrorType,
    >,
    /// Whether this table is synced automatically.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributesFileMetadataCloudStorage {
    pub fn new() -> TableResultV2DataAttributesFileMetadataCloudStorage {
        TableResultV2DataAttributesFileMetadataCloudStorage {
            access_details: None,
            error_message: None,
            error_row_count: None,
            error_type: None,
            sync_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn access_details(
        mut self,
        value: crate::datadogV2::model::TableResultV2DataAttributesFileMetadataOneOfAccessDetails,
    ) -> Self {
        self.access_details = Some(value);
        self
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn error_row_count(mut self, value: i64) -> Self {
        self.error_row_count = Some(value);
        self
    }

    pub fn error_type(
        mut self,
        value: crate::datadogV2::model::TableResultV2DataAttributesFileMetadataCloudStorageErrorType,
    ) -> Self {
        self.error_type = Some(value);
        self
    }

    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
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

impl Default for TableResultV2DataAttributesFileMetadataCloudStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributesFileMetadataCloudStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesFileMetadataCloudStorageVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesFileMetadataCloudStorageVisitor {
            type Value = TableResultV2DataAttributesFileMetadataCloudStorage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_details: Option<crate::datadogV2::model::TableResultV2DataAttributesFileMetadataOneOfAccessDetails> = None;
                let mut error_message: Option<String> = None;
                let mut error_row_count: Option<i64> = None;
                let mut error_type: Option<crate::datadogV2::model::TableResultV2DataAttributesFileMetadataCloudStorageErrorType> = None;
                let mut sync_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_details" => {
                            if v.is_null() {
                                continue;
                            }
                            access_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_row_count" => {
                            if v.is_null() {
                                continue;
                            }
                            error_row_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_type" => {
                            if v.is_null() {
                                continue;
                            }
                            error_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _error_type) = error_type {
                                match _error_type {
                                    crate::datadogV2::model::TableResultV2DataAttributesFileMetadataCloudStorageErrorType::UnparsedObject(_error_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sync_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TableResultV2DataAttributesFileMetadataCloudStorage {
                    access_details,
                    error_message,
                    error_row_count,
                    error_type,
                    sync_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableResultV2DataAttributesFileMetadataCloudStorageVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata specifying where and how to access the reference table's data file.
///
/// For cloud storage tables (S3/GCS/Azure):
///   - sync_enabled and access_details will always be present
///   - error fields (error_message, error_row_count, error_type) are present only when errors occur
///
/// For local file tables:
///   - error fields (error_message, error_row_count) are present only when errors occur
///   - sync_enabled, access_details are never present
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributesFileMetadata {
    /// Cloud storage access configuration for the reference table data file.
    #[serde(rename = "access_details")]
    pub access_details:
        Option<crate::datadogV2::model::TableResultV2DataAttributesFileMetadataOneOfAccessDetails>,
    /// The error message returned from the last operation (sync for cloud storage, upload for local file).
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// The number of rows that failed to process.
    #[serde(rename = "error_row_count")]
    pub error_row_count: Option<i64>,
    /// The type of error that occurred during file processing. This field provides high-level error categories for easier troubleshooting and is only present when there are errors.
    #[serde(rename = "error_type")]
    pub error_type: Option<
        crate::datadogV2::model::TableResultV2DataAttributesFileMetadataCloudStorageErrorType,
    >,
    /// Whether this table is synced automatically from cloud storage. Only applicable for cloud storage sources.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributesFileMetadata {
    pub fn new() -> TableResultV2DataAttributesFileMetadata {
        TableResultV2DataAttributesFileMetadata {
            access_details: None,
            error_message: None,
            error_row_count: None,
            error_type: None,
            sync_enabled: None,
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
}

impl Default for TableResultV2DataAttributesFileMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributesFileMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesFileMetadataVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesFileMetadataVisitor {
            type Value = TableResultV2DataAttributesFileMetadata;

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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TableResultV2DataAttributesFileMetadata {
                    access_details,
                    error_message,
                    error_row_count,
                    error_type,
                    sync_enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableResultV2DataAttributesFileMetadataVisitor)
    }
}

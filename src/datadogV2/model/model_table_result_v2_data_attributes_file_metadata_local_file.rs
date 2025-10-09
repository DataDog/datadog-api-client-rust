// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// File metadata for reference tables created by upload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributesFileMetadataLocalFile {
    /// The error message returned from the creation/update.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// The number of rows that failed to create/update.
    #[serde(rename = "error_row_count")]
    pub error_row_count: Option<i64>,
    /// The upload ID that was used to create/update the table.
    #[serde(rename = "upload_id")]
    pub upload_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributesFileMetadataLocalFile {
    pub fn new() -> TableResultV2DataAttributesFileMetadataLocalFile {
        TableResultV2DataAttributesFileMetadataLocalFile {
            error_message: None,
            error_row_count: None,
            upload_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn error_row_count(mut self, value: i64) -> Self {
        self.error_row_count = Some(value);
        self
    }

    pub fn upload_id(mut self, value: String) -> Self {
        self.upload_id = Some(value);
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

impl Default for TableResultV2DataAttributesFileMetadataLocalFile {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributesFileMetadataLocalFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesFileMetadataLocalFileVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesFileMetadataLocalFileVisitor {
            type Value = TableResultV2DataAttributesFileMetadataLocalFile;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_message: Option<String> = None;
                let mut error_row_count: Option<i64> = None;
                let mut upload_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "upload_id" => {
                            if v.is_null() {
                                continue;
                            }
                            upload_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TableResultV2DataAttributesFileMetadataLocalFile {
                    error_message,
                    error_row_count,
                    upload_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableResultV2DataAttributesFileMetadataLocalFileVisitor)
    }
}

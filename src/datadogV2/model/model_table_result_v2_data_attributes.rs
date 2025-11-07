// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that define the reference table's configuration and properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributes {
    /// UUID of the user who created the reference table.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Optional text describing the purpose or contents of this reference table.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Metadata specifying where and how to access the reference table's data file.
    #[serde(rename = "file_metadata")]
    pub file_metadata: Option<crate::datadogV2::model::TableResultV2DataAttributesFileMetadata>,
    /// UUID of the user who last updated the reference table.
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: Option<String>,
    /// The number of successfully processed rows in the reference table.
    #[serde(rename = "row_count")]
    pub row_count: Option<i64>,
    /// Schema defining the structure and columns of the reference table.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::TableResultV2DataAttributesSchema>,
    /// The source type for reference table data. Includes all possible source types that can appear in responses.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::ReferenceTableSourceType>,
    /// The processing status of the table.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Unique name to identify this reference table. Used in enrichment processors and API calls.
    #[serde(rename = "table_name")]
    pub table_name: Option<String>,
    /// Tags for organizing and filtering reference tables.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// When the reference table was last updated, in ISO 8601 format.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributes {
    pub fn new() -> TableResultV2DataAttributes {
        TableResultV2DataAttributes {
            created_by: None,
            description: None,
            file_metadata: None,
            last_updated_by: None,
            row_count: None,
            schema: None,
            source: None,
            status: None,
            table_name: None,
            tags: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn file_metadata(
        mut self,
        value: crate::datadogV2::model::TableResultV2DataAttributesFileMetadata,
    ) -> Self {
        self.file_metadata = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: String) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn schema(
        mut self,
        value: crate::datadogV2::model::TableResultV2DataAttributesSchema,
    ) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn source(mut self, value: crate::datadogV2::model::ReferenceTableSourceType) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn table_name(mut self, value: String) -> Self {
        self.table_name = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
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

impl Default for TableResultV2DataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesVisitor {
            type Value = TableResultV2DataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut file_metadata: Option<
                    crate::datadogV2::model::TableResultV2DataAttributesFileMetadata,
                > = None;
                let mut last_updated_by: Option<String> = None;
                let mut row_count: Option<i64> = None;
                let mut schema: Option<crate::datadogV2::model::TableResultV2DataAttributesSchema> =
                    None;
                let mut source: Option<crate::datadogV2::model::ReferenceTableSourceType> = None;
                let mut status: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            file_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _file_metadata) = file_metadata {
                                match _file_metadata {
                                    crate::datadogV2::model::TableResultV2DataAttributesFileMetadata::UnparsedObject(_file_metadata) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "last_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "row_count" => {
                            if v.is_null() {
                                continue;
                            }
                            row_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::ReferenceTableSourceType::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            if v.is_null() {
                                continue;
                            }
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TableResultV2DataAttributes {
                    created_by,
                    description,
                    file_metadata,
                    last_updated_by,
                    row_count,
                    schema,
                    source,
                    status,
                    table_name,
                    tags,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableResultV2DataAttributesVisitor)
    }
}

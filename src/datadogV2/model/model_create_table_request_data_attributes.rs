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
pub struct CreateTableRequestDataAttributes {
    /// Optional text describing the purpose or contents of this reference table.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Metadata specifying where and how to access the reference table's data file.
    #[serde(rename = "file_metadata")]
    pub file_metadata:
        Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadata>,
    /// Schema defining the structure and columns of the reference table.
    #[serde(rename = "schema")]
    pub schema: crate::datadogV2::model::CreateTableRequestDataAttributesSchema,
    /// The source type for creating reference table data. Only these source types can be created through this API.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::ReferenceTableCreateSourceType,
    /// Name to identify this reference table.
    #[serde(rename = "table_name")]
    pub table_name: String,
    /// Tags for organizing and filtering reference tables.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTableRequestDataAttributes {
    pub fn new(
        schema: crate::datadogV2::model::CreateTableRequestDataAttributesSchema,
        source: crate::datadogV2::model::ReferenceTableCreateSourceType,
        table_name: String,
    ) -> CreateTableRequestDataAttributes {
        CreateTableRequestDataAttributes {
            description: None,
            file_metadata: None,
            schema,
            source,
            table_name,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn file_metadata(
        mut self,
        value: crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadata,
    ) -> Self {
        self.file_metadata = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for CreateTableRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateTableRequestDataAttributesVisitor {
            type Value = CreateTableRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut file_metadata: Option<
                    crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadata,
                > = None;
                let mut schema: Option<
                    crate::datadogV2::model::CreateTableRequestDataAttributesSchema,
                > = None;
                let mut source: Option<crate::datadogV2::model::ReferenceTableCreateSourceType> =
                    None;
                let mut table_name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadata::UnparsedObject(_file_metadata) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "schema" => {
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::ReferenceTableCreateSourceType::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let schema = schema.ok_or_else(|| M::Error::missing_field("schema"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = CreateTableRequestDataAttributes {
                    description,
                    file_metadata,
                    schema,
                    source,
                    table_name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateTableRequestDataAttributesVisitor)
    }
}

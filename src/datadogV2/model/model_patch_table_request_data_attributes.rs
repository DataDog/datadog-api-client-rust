// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that define the updates to the reference table's configuration and properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchTableRequestDataAttributes {
    /// Optional text describing the purpose or contents of this reference table.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Metadata specifying where and how to access the reference table's data file.
    #[serde(rename = "file_metadata")]
    pub file_metadata: Option<crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadata>,
    /// Schema defining the updates to the structure and columns of the reference table. Schema fields cannot be deleted or renamed.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::PatchTableRequestDataAttributesSchema>,
    /// Whether this table is synced automatically.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: Option<bool>,
    /// Tags for organizing and filtering reference tables.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchTableRequestDataAttributes {
    pub fn new() -> PatchTableRequestDataAttributes {
        PatchTableRequestDataAttributes {
            description: None,
            file_metadata: None,
            schema: None,
            sync_enabled: None,
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
        value: crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadata,
    ) -> Self {
        self.file_metadata = Some(value);
        self
    }

    pub fn schema(
        mut self,
        value: crate::datadogV2::model::PatchTableRequestDataAttributesSchema,
    ) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
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

impl Default for PatchTableRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchTableRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchTableRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for PatchTableRequestDataAttributesVisitor {
            type Value = PatchTableRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut file_metadata: Option<
                    crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadata,
                > = None;
                let mut schema: Option<
                    crate::datadogV2::model::PatchTableRequestDataAttributesSchema,
                > = None;
                let mut sync_enabled: Option<bool> = None;
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
                                    crate::datadogV2::model::PatchTableRequestDataAttributesFileMetadata::UnparsedObject(_file_metadata) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = PatchTableRequestDataAttributes {
                    description,
                    file_metadata,
                    schema,
                    sync_enabled,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PatchTableRequestDataAttributesVisitor)
    }
}

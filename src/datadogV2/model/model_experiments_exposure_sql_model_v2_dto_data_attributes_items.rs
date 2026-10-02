// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Property column available from the exposure SQL model.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExposureSQLModelV2DTODataAttributesItems {
    /// SQL result column that contains this property.
    #[serde(rename = "column_name")]
    pub column_name: Option<String>,
    /// Data type of the property column.
    #[serde(rename = "column_type")]
    pub column_type: Option<String>,
    /// Description of the exposure property.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Identifier of the exposure property.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the exposure property.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Suffix used for this property column in the analysis pipeline.
    #[serde(rename = "pipeline_column_suffix")]
    pub pipeline_column_suffix: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExposureSQLModelV2DTODataAttributesItems {
    pub fn new() -> ExperimentsExposureSQLModelV2DTODataAttributesItems {
        ExperimentsExposureSQLModelV2DTODataAttributesItems {
            column_name: None,
            column_type: None,
            description: None,
            id: None,
            migration_metadata: None,
            name: None,
            pipeline_column_suffix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column_name(mut self, value: String) -> Self {
        self.column_name = Some(value);
        self
    }

    pub fn column_type(mut self, value: String) -> Self {
        self.column_type = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pipeline_column_suffix(mut self, value: String) -> Self {
        self.pipeline_column_suffix = Some(value);
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

impl Default for ExperimentsExposureSQLModelV2DTODataAttributesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsExposureSQLModelV2DTODataAttributesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExposureSQLModelV2DTODataAttributesItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsExposureSQLModelV2DTODataAttributesItemsVisitor {
            type Value = ExperimentsExposureSQLModelV2DTODataAttributesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut column_type: Option<String> = None;
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut pipeline_column_suffix: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_name" => {
                            if v.is_null() {
                                continue;
                            }
                            column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_type" => {
                            if v.is_null() {
                                continue;
                            }
                            column_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_column_suffix" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_column_suffix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsExposureSQLModelV2DTODataAttributesItems {
                    column_name,
                    column_type,
                    description,
                    id,
                    migration_metadata,
                    name,
                    pipeline_column_suffix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExposureSQLModelV2DTODataAttributesItemsVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A measure available from a metric SQL model column.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
    /// Name of the SQL result column that supplies this measure.
    #[serde(rename = "column_name")]
    pub column_name: Option<String>,
    /// Data type of a column in the SQL model.
    #[serde(rename = "column_type")]
    pub column_type: Option<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType>,
    /// Text that explains the measure.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// ID of the measure.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the measure.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
    pub fn new() -> ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
        ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
            column_name: None,
            column_type: None,
            description: None,
            id: None,
            migration_metadata: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column_name(mut self, value: String) -> Self {
        self.column_name = Some(value);
        self
    }

    pub fn column_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType,
    ) -> Self {
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItemsVisitor {
            type Value = ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut column_type: Option<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType> = None;
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
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
                            if let Some(ref _column_type) = column_type {
                                match _column_type {
                                    crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType::UnparsedObject(_column_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems {
                    column_name,
                    column_type,
                    description,
                    id,
                    migration_metadata,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItemsVisitor)
    }
}

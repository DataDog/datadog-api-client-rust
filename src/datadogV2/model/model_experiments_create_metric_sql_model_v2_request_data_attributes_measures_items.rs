// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Column in the SQL model that supplies values for a metric measure.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
    /// SQL result column that contains the measure values.
    #[serde(rename = "column_name")]
    pub column_name: String,
    /// Data type of a column in the SQL model.
    #[serde(rename = "column_type")]
    pub column_type: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType,
    /// Description of the measure.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the measure.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
    pub fn new(
        column_name: String,
        column_type: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType,
    ) -> ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
        ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
            column_name,
            column_type,
            description: None,
            migration_metadata: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
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

impl<'de> Deserialize<'de> for ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItemsVisitor
        {
            type Value = ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut column_type: Option<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType> = None;
                let mut description: Option<Option<String>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_name" => {
                            column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_type" => {
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
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let column_name =
                    column_name.ok_or_else(|| M::Error::missing_field("column_name"))?;
                let column_type =
                    column_type.ok_or_else(|| M::Error::missing_field("column_type"))?;

                let content = ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems {
                    column_name,
                    column_type,
                    description,
                    migration_metadata,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItemsVisitor,
        )
    }
}

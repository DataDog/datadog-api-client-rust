// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Complete column mappings and query used to replace the metric SQL model.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
    /// SQL column used to partition the source data by date.
    #[serde(rename = "date_partition_column", default, with = "::serde_with::rust::double_option")]
    pub date_partition_column: Option<Option<String>>,
    /// Text that explains the metric SQL model.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Measures available from the SQL model's result columns.
    #[serde(rename = "measures")]
    pub measures: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems>>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the metric SQL model.
    #[serde(rename = "name")]
    pub name: String,
    /// Property columns exposed by the SQL model.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::ExperimentsMetricSQLModelPropertyInput>>,
    /// SQL query that produces the model's source data.
    #[serde(rename = "sql")]
    pub sql: String,
    /// Subject types mapped to columns in the SQL model.
    #[serde(rename = "subject_types")]
    pub subject_types: Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>,
    /// SQL column that supplies the event timestamp.
    #[serde(rename = "timestamp_column")]
    pub timestamp_column: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
    pub fn new(
        name: String,
        sql: String,
        subject_types: Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>,
        timestamp_column: String,
    ) -> ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
        ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
            date_partition_column: None,
            description: None,
            measures: None,
            migration_metadata: None,
            name,
            properties: None,
            sql,
            subject_types,
            timestamp_column,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn date_partition_column(mut self, value: Option<String>) -> Self {
        self.date_partition_column = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn measures(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems>,
    ) -> Self {
        self.measures = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsMetricSQLModelPropertyInput>,
    ) -> Self {
        self.properties = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsUpdateMetricSQLModelV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsUpdateMetricSQLModelV2RequestDataAttributesVisitor {
            type Value = ExperimentsUpdateMetricSQLModelV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut date_partition_column: Option<Option<String>> = None;
                let mut description: Option<Option<String>> = None;
                let mut measures: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut properties: Option<
                    Vec<crate::datadogV2::model::ExperimentsMetricSQLModelPropertyInput>,
                > = None;
                let mut sql: Option<String> = None;
                let mut subject_types: Option<Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>> = None;
                let mut timestamp_column: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "date_partition_column" => {
                            date_partition_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "measures" => {
                            if v.is_null() {
                                continue;
                            }
                            measures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sql" => {
                            sql = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_types" => {
                            subject_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_column" => {
                            timestamp_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let sql = sql.ok_or_else(|| M::Error::missing_field("sql"))?;
                let subject_types =
                    subject_types.ok_or_else(|| M::Error::missing_field("subject_types"))?;
                let timestamp_column =
                    timestamp_column.ok_or_else(|| M::Error::missing_field("timestamp_column"))?;

                let content = ExperimentsUpdateMetricSQLModelV2RequestDataAttributes {
                    date_partition_column,
                    description,
                    measures,
                    migration_metadata,
                    name,
                    properties,
                    sql,
                    subject_types,
                    timestamp_column,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsUpdateMetricSQLModelV2RequestDataAttributesVisitor)
    }
}

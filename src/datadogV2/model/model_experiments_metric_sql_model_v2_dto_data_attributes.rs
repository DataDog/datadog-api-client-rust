// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the metric SQL model.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricSQLModelV2DTODataAttributes {
    /// Time when this resource was certified.
    #[serde(
        rename = "certified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub certified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time when this resource was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// SQL column used to partition the source data by date.
    #[serde(
        rename = "date_partition_column",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub date_partition_column: Option<Option<String>>,
    /// Text that explains the metric SQL model.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Read-only measure ID. Pass it as warehouse_metric_measure.id when the metric operation is count.
    #[serde(rename = "event_count_measure_id")]
    pub event_count_measure_id: Option<String>,
    /// Number of experiments that reference this resource.
    #[serde(
        rename = "experiment_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_count: Option<Option<i64>>,
    /// Whether this resource has been certified.
    #[serde(rename = "is_certified")]
    pub is_certified: Option<bool>,
    /// Measures available from the SQL model's result columns.
    #[serde(rename = "measures")]
    pub measures: Option<
        Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems>,
    >,
    /// Number of metrics that use this SQL model.
    #[serde(
        rename = "metric_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metric_count: Option<Option<i64>>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the metric SQL model.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Property columns exposed by the SQL model.
    #[serde(rename = "properties")]
    pub properties: Option<
        Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems>,
    >,
    /// SQL query that produces the model's source data.
    #[serde(rename = "sql")]
    pub sql: Option<String>,
    /// Subject types mapped to columns in the SQL model.
    #[serde(rename = "subject_types")]
    pub subject_types: Option<
        Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems>,
    >,
    /// SQL column that supplies the event timestamp.
    #[serde(rename = "timestamp_column")]
    pub timestamp_column: Option<String>,
    /// Time when this resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricSQLModelV2DTODataAttributes {
    pub fn new() -> ExperimentsMetricSQLModelV2DTODataAttributes {
        ExperimentsMetricSQLModelV2DTODataAttributes {
            certified_at: None,
            created_at: None,
            date_partition_column: None,
            description: None,
            event_count_measure_id: None,
            experiment_count: None,
            is_certified: None,
            measures: None,
            metric_count: None,
            migration_metadata: None,
            name: None,
            properties: None,
            sql: None,
            subject_types: None,
            timestamp_column: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn certified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.certified_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn date_partition_column(mut self, value: Option<String>) -> Self {
        self.date_partition_column = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn event_count_measure_id(mut self, value: String) -> Self {
        self.event_count_measure_id = Some(value);
        self
    }

    pub fn experiment_count(mut self, value: Option<i64>) -> Self {
        self.experiment_count = Some(value);
        self
    }

    pub fn is_certified(mut self, value: bool) -> Self {
        self.is_certified = Some(value);
        self
    }

    pub fn measures(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems,
        >,
    ) -> Self {
        self.measures = Some(value);
        self
    }

    pub fn metric_count(mut self, value: Option<i64>) -> Self {
        self.metric_count = Some(value);
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

    pub fn properties(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems,
        >,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn sql(mut self, value: String) -> Self {
        self.sql = Some(value);
        self
    }

    pub fn subject_types(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems,
        >,
    ) -> Self {
        self.subject_types = Some(value);
        self
    }

    pub fn timestamp_column(mut self, value: String) -> Self {
        self.timestamp_column = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
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

impl Default for ExperimentsMetricSQLModelV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricSQLModelV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricSQLModelV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricSQLModelV2DTODataAttributesVisitor {
            type Value = ExperimentsMetricSQLModelV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut certified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut date_partition_column: Option<Option<String>> = None;
                let mut description: Option<Option<String>> = None;
                let mut event_count_measure_id: Option<String> = None;
                let mut experiment_count: Option<Option<i64>> = None;
                let mut is_certified: Option<bool> = None;
                let mut measures: Option<Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems>> = None;
                let mut metric_count: Option<Option<i64>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut properties: Option<Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesMeasuresItems>> = None;
                let mut sql: Option<String> = None;
                let mut subject_types: Option<Vec<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems>> = None;
                let mut timestamp_column: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "certified_at" => {
                            certified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date_partition_column" => {
                            date_partition_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_count_measure_id" => {
                            if v.is_null() {
                                continue;
                            }
                            event_count_measure_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_count" => {
                            experiment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_certified" => {
                            if v.is_null() {
                                continue;
                            }
                            is_certified =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "measures" => {
                            if v.is_null() {
                                continue;
                            }
                            measures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_count" => {
                            metric_count =
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
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sql" => {
                            if v.is_null() {
                                continue;
                            }
                            sql = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_types" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_column" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ExperimentsMetricSQLModelV2DTODataAttributes {
                    certified_at,
                    created_at,
                    date_partition_column,
                    description,
                    event_count_measure_id,
                    experiment_count,
                    is_certified,
                    measures,
                    metric_count,
                    migration_metadata,
                    name,
                    properties,
                    sql,
                    subject_types,
                    timestamp_column,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsMetricSQLModelV2DTODataAttributesVisitor)
    }
}

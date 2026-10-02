// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query and column mappings used to read experiment assignment data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExposureSQLModelV2DTODataAttributes {
    /// Time when the exposure SQL model was archived.
    #[serde(rename = "archived_at", default, with = "::serde_with::rust::double_option")]
    pub archived_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time when the exposure SQL model was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Column used to identify date partitions in the exposure data.
    #[serde(rename = "date_partition_column", default, with = "::serde_with::rust::double_option")]
    pub date_partition_column: Option<Option<String>>,
    /// SQL result column that contains the experiment key.
    #[serde(rename = "experiment_column")]
    pub experiment_column: Option<String>,
    /// Number of experiments associated with the exposure SQL model.
    #[serde(rename = "experiment_count", default, with = "::serde_with::rust::double_option")]
    pub experiment_count: Option<Option<i64>>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the exposure SQL model.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Property columns available for filtering or splitting exposure data.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTODataAttributesItems>>,
    /// SQL query that supplies the experiment assignment data.
    #[serde(rename = "sql")]
    pub sql: Option<String>,
    /// Mappings between subject types and their identifier columns.
    #[serde(rename = "subject_types")]
    pub subject_types: Option<Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>>,
    /// SQL result column that contains the assignment timestamp.
    #[serde(rename = "timestamp_column")]
    pub timestamp_column: Option<String>,
    /// Time when the exposure SQL model was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// SQL result column that contains the assigned variant.
    #[serde(rename = "variant_column")]
    pub variant_column: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsExposureSQLModelV2DTODataAttributes {
    pub fn new() -> ExperimentsExposureSQLModelV2DTODataAttributes {
        ExperimentsExposureSQLModelV2DTODataAttributes {
            archived_at: None,
            created_at: None,
            date_partition_column: None,
            experiment_column: None,
            experiment_count: None,
            migration_metadata: None,
            name: None,
            properties: None,
            sql: None,
            subject_types: None,
            timestamp_column: None,
            updated_at: None,
            variant_column: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.archived_at = Some(value);
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

    pub fn experiment_column(mut self, value: String) -> Self {
        self.experiment_column = Some(value);
        self
    }

    pub fn experiment_count(mut self, value: Option<i64>) -> Self {
        self.experiment_count = Some(value);
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
        value: Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTODataAttributesItems>,
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
        value: Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>,
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

    pub fn variant_column(mut self, value: String) -> Self {
        self.variant_column = Some(value);
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

impl Default for ExperimentsExposureSQLModelV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsExposureSQLModelV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExposureSQLModelV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsExposureSQLModelV2DTODataAttributesVisitor {
            type Value = ExperimentsExposureSQLModelV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut date_partition_column: Option<Option<String>> = None;
                let mut experiment_column: Option<String> = None;
                let mut experiment_count: Option<Option<i64>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut properties: Option<Vec<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTODataAttributesItems>> = None;
                let mut sql: Option<String> = None;
                let mut subject_types: Option<Vec<crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems>> = None;
                let mut timestamp_column: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut variant_column: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            archived_at =
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
                        "experiment_column" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_count" => {
                            experiment_count =
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
                        "variant_column" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsExposureSQLModelV2DTODataAttributes {
                    archived_at,
                    created_at,
                    date_partition_column,
                    experiment_column,
                    experiment_count,
                    migration_metadata,
                    name,
                    properties,
                    sql,
                    subject_types,
                    timestamp_column,
                    updated_at,
                    variant_column,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExposureSQLModelV2DTODataAttributesVisitor)
    }
}

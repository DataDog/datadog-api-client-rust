// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the subject type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsSubjectTypeV2DTODataAttributes {
    /// Time when this resource was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of experiments that reference this resource.
    #[serde(
        rename = "experiment_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_count: Option<Option<i64>>,
    /// Number of exposure sources that reference this subject type.
    #[serde(
        rename = "exposure_source_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub exposure_source_count: Option<Option<i64>>,
    /// Whether this is the organization's default subject type.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// Number of metric SQL models that reference this subject type.
    #[serde(
        rename = "metric_sql_model_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metric_sql_model_count: Option<Option<i64>>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the subject type.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Product Analytics attribute used to identify subjects of this type.
    #[serde(rename = "product_analytics_attribute")]
    pub product_analytics_attribute: Option<String>,
    /// Number of protocols that reference this subject type.
    #[serde(
        rename = "protocol_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub protocol_count: Option<Option<i64>>,
    /// Time when this resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Warehouse columns that identify subjects of this type.
    #[serde(rename = "warehouse_column_names")]
    pub warehouse_column_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsSubjectTypeV2DTODataAttributes {
    pub fn new() -> ExperimentsSubjectTypeV2DTODataAttributes {
        ExperimentsSubjectTypeV2DTODataAttributes {
            created_at: None,
            experiment_count: None,
            exposure_source_count: None,
            is_default: None,
            metric_sql_model_count: None,
            migration_metadata: None,
            name: None,
            product_analytics_attribute: None,
            protocol_count: None,
            updated_at: None,
            warehouse_column_names: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn experiment_count(mut self, value: Option<i64>) -> Self {
        self.experiment_count = Some(value);
        self
    }

    pub fn exposure_source_count(mut self, value: Option<i64>) -> Self {
        self.exposure_source_count = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn metric_sql_model_count(mut self, value: Option<i64>) -> Self {
        self.metric_sql_model_count = Some(value);
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

    pub fn product_analytics_attribute(mut self, value: String) -> Self {
        self.product_analytics_attribute = Some(value);
        self
    }

    pub fn protocol_count(mut self, value: Option<i64>) -> Self {
        self.protocol_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn warehouse_column_names(mut self, value: Vec<String>) -> Self {
        self.warehouse_column_names = Some(value);
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

impl Default for ExperimentsSubjectTypeV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsSubjectTypeV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsSubjectTypeV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsSubjectTypeV2DTODataAttributesVisitor {
            type Value = ExperimentsSubjectTypeV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut experiment_count: Option<Option<i64>> = None;
                let mut exposure_source_count: Option<Option<i64>> = None;
                let mut is_default: Option<bool> = None;
                let mut metric_sql_model_count: Option<Option<i64>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut product_analytics_attribute: Option<String> = None;
                let mut protocol_count: Option<Option<i64>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut warehouse_column_names: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_count" => {
                            experiment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_source_count" => {
                            exposure_source_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_sql_model_count" => {
                            metric_sql_model_count =
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
                        "product_analytics_attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol_count" => {
                            protocol_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_column_names" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse_column_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsSubjectTypeV2DTODataAttributes {
                    created_at,
                    experiment_count,
                    exposure_source_count,
                    is_default,
                    metric_sql_model_count,
                    migration_metadata,
                    name,
                    product_analytics_attribute,
                    protocol_count,
                    updated_at,
                    warehouse_column_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsSubjectTypeV2DTODataAttributesVisitor)
    }
}

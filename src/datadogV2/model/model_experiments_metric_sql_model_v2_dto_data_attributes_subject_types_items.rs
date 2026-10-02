// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A mapping between a subject type and its identifying SQL column.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
    /// Name of the SQL result column that identifies subjects of this type.
    #[serde(rename = "column_name")]
    pub column_name: Option<String>,
    /// ID of the subject type used by this configuration.
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<String>,
    /// Read-only measure ID. Pass it as warehouse_metric_measure.id when the metric operation is `uniqueSubjects`.
    #[serde(rename = "unique_subject_count_measure_id")]
    pub unique_subject_count_measure_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
    pub fn new() -> ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
        ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
            column_name: None,
            subject_type_id: None,
            unique_subject_count_measure_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column_name(mut self, value: String) -> Self {
        self.column_name = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: String) -> Self {
        self.subject_type_id = Some(value);
        self
    }

    pub fn unique_subject_count_measure_id(mut self, value: String) -> Self {
        self.unique_subject_count_measure_id = Some(value);
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

impl Default for ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItemsVisitor {
            type Value = ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut subject_type_id: Option<String> = None;
                let mut unique_subject_count_measure_id: Option<String> = None;
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
                        "subject_type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unique_subject_count_measure_id" => {
                            if v.is_null() {
                                continue;
                            }
                            unique_subject_count_measure_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItems {
                    column_name,
                    subject_type_id,
                    unique_subject_count_measure_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsMetricSQLModelV2DTODataAttributesSubjectTypesItemsVisitor)
    }
}

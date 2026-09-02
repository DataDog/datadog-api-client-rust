// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Measure configuration for one side of a source to target comparison.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
    /// Custom SQL query used to compute the measure for this entity.
    #[serde(rename = "custom_sql")]
    pub custom_sql: Option<String>,
    /// Custom WHERE clause applied when computing the measure for this entity.
    #[serde(rename = "custom_where")]
    pub custom_where: Option<String>,
    /// Identifier of the data entity to measure.
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// Type of the data entity to measure.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// Columns to group results by when computing the measure for this entity.
    #[serde(rename = "group_by_columns")]
    pub group_by_columns: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
    pub fn new(
        entity_id: String,
        entity_type: String,
    ) -> MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
        MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
            custom_sql: None,
            custom_where: None,
            entity_id,
            entity_type,
            group_by_columns: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_sql(mut self, value: String) -> Self {
        self.custom_sql = Some(value);
        self
    }

    pub fn custom_where(mut self, value: String) -> Self {
        self.custom_where = Some(value);
        self
    }

    pub fn group_by_columns(mut self, value: Vec<String>) -> Self {
        self.group_by_columns = Some(value);
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

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualityEntityMetricConfigVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualityEntityMetricConfigVisitor {
            type Value = MonitorFormulaAndFunctionDataQualityEntityMetricConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_sql: Option<String> = None;
                let mut custom_where: Option<String> = None;
                let mut entity_id: Option<String> = None;
                let mut entity_type: Option<String> = None;
                let mut group_by_columns: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_sql" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_sql = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_where" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_where =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_id" => {
                            entity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_type" => {
                            entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let entity_id = entity_id.ok_or_else(|| M::Error::missing_field("entity_id"))?;
                let entity_type =
                    entity_type.ok_or_else(|| M::Error::missing_field("entity_type"))?;

                let content = MonitorFormulaAndFunctionDataQualityEntityMetricConfig {
                    custom_sql,
                    custom_where,
                    entity_id,
                    entity_type,
                    group_by_columns,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataQualityEntityMetricConfigVisitor)
    }
}

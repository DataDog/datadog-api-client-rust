// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions data quality query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualityQueryDefinition {
    /// Data source for data quality queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource,
    /// Filter expression used to match on data entities. Uses Aastra query syntax.
    #[serde(rename = "filter")]
    pub filter: String,
    /// Optional grouping fields for aggregation.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// The data quality measure to query. Common values include:
    /// `bytes`, `cardinality`, `custom`, `freshness`, `max`, `mean`, `min`,
    /// `nullness`, `percent_negative`, `percent_zero`, `row_count`, `stddev`,
    /// `sum`, `uniqueness`. Additional values may be supported.
    #[serde(rename = "measure")]
    pub measure: String,
    /// Monitor configuration options for data quality queries.
    #[serde(rename = "monitor_options")]
    pub monitor_options:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityMonitorOptions>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Schema version for the data quality query.
    #[serde(rename = "schema_version")]
    pub schema_version: Option<String>,
    /// Optional scoping expression to further filter metrics. Uses metrics filter syntax.
    /// This is useful when an entity has been configured to emit metrics with additional tags.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataQualityQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource,
        filter: String,
        measure: String,
        name: String,
    ) -> MonitorFormulaAndFunctionDataQualityQueryDefinition {
        MonitorFormulaAndFunctionDataQualityQueryDefinition {
            data_source,
            filter,
            group_by: None,
            measure,
            monitor_options: None,
            name,
            schema_version: None,
            scope: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn monitor_options(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityMonitorOptions,
    ) -> Self {
        self.monitor_options = Some(value);
        self
    }

    pub fn schema_version(mut self, value: String) -> Self {
        self.schema_version = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualityQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualityQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualityQueryDefinitionVisitor {
            type Value = MonitorFormulaAndFunctionDataQualityQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource,
                > = None;
                let mut filter: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut measure: Option<String> = None;
                let mut monitor_options: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityMonitorOptions,
                > = None;
                let mut name: Option<String> = None;
                let mut schema_version: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "measure" => {
                            measure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_options" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema_version" => {
                            if v.is_null() {
                                continue;
                            }
                            schema_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let measure = measure.ok_or_else(|| M::Error::missing_field("measure"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = MonitorFormulaAndFunctionDataQualityQueryDefinition {
                    data_source,
                    filter,
                    group_by,
                    measure,
                    monitor_options,
                    name,
                    schema_version,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataQualityQueryDefinitionVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions aggregate filtered query. Used to filter base query results using data from another source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
    /// Base query for aggregate queries. Can be an events query or a metrics query.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
    /// Compute options for the query.
    #[serde(rename = "compute")]
    pub compute:
        Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>>,
    /// Data source for aggregate filtered queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredDataSource,
    /// Filter query for aggregate filtered queries. Can be an events query or a reference table query.
    #[serde(rename = "filter_query")]
    pub filter_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilterQuery,
    /// Filter conditions for the query.
    #[serde(rename = "filters")]
    pub filters: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryFilter>,
    /// Group by options for the query.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
    pub fn new(
        base_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
        data_source: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredDataSource,
        filter_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilterQuery,
        filters: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryFilter>,
    ) -> MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
        MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
            base_query,
            compute: None,
            data_source,
            filter_query,
            filters,
            group_by: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn compute(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>,
    ) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionAggregateFilteredQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionAggregateFilteredQueryDefinitionVisitor {
            type Value = MonitorFormulaAndFunctionAggregateFilteredQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
                > = None;
                let mut compute: Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>> = None;
                let mut data_source: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredDataSource,
                > = None;
                let mut filter_query: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilterQuery,
                > = None;
                let mut filters: Option<
                    Vec<crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryFilter>,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
                > = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _base_query) = base_query {
                                match _base_query {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery::UnparsedObject(_base_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filter_query" => {
                            filter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filter_query) = filter_query {
                                match _filter_query {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateFilterQuery::UnparsedObject(_filter_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let filter_query =
                    filter_query.ok_or_else(|| M::Error::missing_field("filter_query"))?;
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;

                let content = MonitorFormulaAndFunctionAggregateFilteredQueryDefinition {
                    base_query,
                    compute,
                    data_source,
                    filter_query,
                    filters,
                    group_by,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(MonitorFormulaAndFunctionAggregateFilteredQueryDefinitionVisitor)
    }
}

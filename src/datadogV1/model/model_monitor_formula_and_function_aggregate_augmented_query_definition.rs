// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions aggregate augmented query. Used to enrich base query results with data from a reference table.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
    /// Augment query for aggregate augmented queries. Can be an events query or a reference table query.
    #[serde(rename = "augment_query")]
    pub augment_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentQuery,
    /// Base query for aggregate queries. Can be an events query or a metrics query.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
    /// Compute options for the query.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>,
    /// Data source for aggregate augmented queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedDataSource,
    /// Group by options for the query.
    #[serde(rename = "group_by")]
    pub group_by: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
    /// Join condition for aggregate augmented queries.
    #[serde(rename = "join_condition")]
    pub join_condition:
        crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinCondition,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
    pub fn new(
        augment_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentQuery,
        base_query: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
        compute: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>,
        data_source: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedDataSource,
        group_by: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
        join_condition: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinCondition,
    ) -> MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
        MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
            augment_query,
            base_query,
            compute,
            data_source,
            group_by,
            join_condition,
            name: None,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionAggregateAugmentedQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionAggregateAugmentedQueryDefinitionVisitor {
            type Value = MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut augment_query: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentQuery,
                > = None;
                let mut base_query: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery,
                > = None;
                let mut compute: Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute>> = None;
                let mut data_source: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedDataSource,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
                > = None;
                let mut join_condition: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinCondition,
                > = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "augment_query" => {
                            augment_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _augment_query) = augment_query {
                                match _augment_query {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentQuery::UnparsedObject(_augment_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
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
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_condition" => {
                            join_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let augment_query =
                    augment_query.ok_or_else(|| M::Error::missing_field("augment_query"))?;
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let join_condition =
                    join_condition.ok_or_else(|| M::Error::missing_field("join_condition"))?;

                let content = MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition {
                    augment_query,
                    base_query,
                    compute,
                    data_source,
                    group_by,
                    join_condition,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(MonitorFormulaAndFunctionAggregateAugmentedQueryDefinitionVisitor)
    }
}

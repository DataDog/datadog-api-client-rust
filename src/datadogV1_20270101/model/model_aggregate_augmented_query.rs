// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregateAugmentedQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "augment_query")]
    pub augment_query: crate::datadogV1_20270101::model::CompositeSourceSubQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1_20270101::model::CompositeSourceBaseQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV1_20270101::model::CompositeCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::AggregateAugmentedQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::CompositeGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_condition")]
    pub join_condition: crate::datadogV1_20270101::model::AggregateAugmentedQueryJoinCondition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AggregateAugmentedQuery {
    pub fn new(augment_query: crate::datadogV1_20270101::model::CompositeSourceSubQuery, base_query: crate::datadogV1_20270101::model::CompositeSourceBaseQuery, compute: Vec<crate::datadogV1_20270101::model::CompositeCompute>, data_source: crate::datadogV1_20270101::model::AggregateAugmentedQueryDataSource, join_condition: crate::datadogV1_20270101::model::AggregateAugmentedQueryJoinCondition, name: String) -> AggregateAugmentedQuery {
        AggregateAugmentedQuery {
            augment_query,
            base_query,
            compute,
            data_source,
            group_by: None,
            join_condition,
            name,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::CompositeGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for AggregateAugmentedQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregateAugmentedQueryVisitor;
        impl<'a> Visitor<'a> for AggregateAugmentedQueryVisitor {
            type Value = AggregateAugmentedQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut augment_query: Option<crate::datadogV1_20270101::model::CompositeSourceSubQuery> = None;
                let mut base_query: Option<crate::datadogV1_20270101::model::CompositeSourceBaseQuery> = None;
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::CompositeCompute>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::AggregateAugmentedQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::CompositeGroupBy>> = None;
                let mut join_condition: Option<crate::datadogV1_20270101::model::AggregateAugmentedQueryJoinCondition> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "augment_query" => {
                            augment_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _augment_query) = augment_query {
                                match _augment_query {
                                    crate::datadogV1_20270101::model::CompositeSourceSubQuery::UnparsedObject(_augment_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _base_query) = base_query {
                                match _base_query {
                                    crate::datadogV1_20270101::model::CompositeSourceBaseQuery::UnparsedObject(_base_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::AggregateAugmentedQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "join_condition" => {
                            join_condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let augment_query = augment_query.ok_or_else(|| M::Error::missing_field("augment_query"))?;
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let join_condition = join_condition.ok_or_else(|| M::Error::missing_field("join_condition"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = AggregateAugmentedQuery {
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

        deserializer.deserialize_any(AggregateAugmentedQueryVisitor)
    }
}

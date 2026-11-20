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
pub struct FormulaProductAnalyticsExtendedQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::ProductAnalyticsCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::ProductAnalyticsGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<crate::datadogV1::model::FormulaProductAnalyticsExtendedQuery20270101Indexes>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::BaseProductAnalyticsQuery,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaProductAnalyticsExtendedQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::ProductAnalyticsCompute, data_source: crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQueryDataSource, name: String, query: crate::datadogV1_20270101::model::BaseProductAnalyticsQuery) -> FormulaProductAnalyticsExtendedQuery {
        FormulaProductAnalyticsExtendedQuery {
            audience_filters: None,
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name,
            query,
            _unparsed: false,
        }
    }
    
    pub fn audience_filters(mut self, value: crate::datadogV1_20270101::model::AudienceFilters) -> Self {
        self.audience_filters = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::ProductAnalyticsGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: crate::datadogV1::model::FormulaProductAnalyticsExtendedQuery20270101Indexes) -> Self {
        self.indexes = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaProductAnalyticsExtendedQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaProductAnalyticsExtendedQueryVisitor;
        impl<'a> Visitor<'a> for FormulaProductAnalyticsExtendedQueryVisitor {
            type Value = FormulaProductAnalyticsExtendedQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<crate::datadogV1_20270101::model::AudienceFilters> = None;
                let mut compute: Option<crate::datadogV1_20270101::model::ProductAnalyticsCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::ProductAnalyticsGroupBy>> = None;
                let mut indexes: Option<crate::datadogV1::model::FormulaProductAnalyticsExtendedQuery20270101Indexes> = None;
                let mut name: Option<String> = None;
                let mut query: Option<crate::datadogV1_20270101::model::BaseProductAnalyticsQuery> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQueryDataSource::UnparsedObject(_data_source) => {
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
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _indexes) = indexes {
                                match _indexes {
                                    crate::datadogV1::model::FormulaProductAnalyticsExtendedQuery20270101Indexes::UnparsedObject(_indexes) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV1_20270101::model::BaseProductAnalyticsQuery::UnparsedObject(_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = FormulaProductAnalyticsExtendedQuery {
                    audience_filters,
                    compute,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaProductAnalyticsExtendedQueryVisitor)
    }
}

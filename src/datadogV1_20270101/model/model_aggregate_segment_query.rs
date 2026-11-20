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
pub struct AggregateSegmentQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1_20270101::model::CompositeEventsPlatformQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Vec<crate::datadogV1_20270101::model::CompositeCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::AggregateSegmentQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::CompositeGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "segment_query")]
    pub segment_query: crate::datadogV1_20270101::model::AggregateSegmentQuerySegmentQuery,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AggregateSegmentQuery {
    pub fn new(base_query: crate::datadogV1_20270101::model::CompositeEventsPlatformQuery, compute: Vec<crate::datadogV1_20270101::model::CompositeCompute>, data_source: crate::datadogV1_20270101::model::AggregateSegmentQueryDataSource, name: String, segment_query: crate::datadogV1_20270101::model::AggregateSegmentQuerySegmentQuery) -> AggregateSegmentQuery {
        AggregateSegmentQuery {
            base_query,
            compute,
            data_source,
            group_by: None,
            name,
            segment_query,
            _unparsed: false,
        }
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::CompositeGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for AggregateSegmentQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregateSegmentQueryVisitor;
        impl<'a> Visitor<'a> for AggregateSegmentQueryVisitor {
            type Value = AggregateSegmentQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformQuery> = None;
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::CompositeCompute>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::AggregateSegmentQueryDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::CompositeGroupBy>> = None;
                let mut name: Option<String> = None;
                let mut segment_query: Option<crate::datadogV1_20270101::model::AggregateSegmentQuerySegmentQuery> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::AggregateSegmentQueryDataSource::UnparsedObject(_data_source) => {
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "segment_query" => {
                            segment_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let segment_query = segment_query.ok_or_else(|| M::Error::missing_field("segment_query"))?;

                let content = AggregateSegmentQuery {
                    base_query,
                    compute,
                    data_source,
                    group_by,
                    name,
                    segment_query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregateSegmentQueryVisitor)
    }
}

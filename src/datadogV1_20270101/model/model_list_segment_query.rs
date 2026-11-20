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
pub struct ListSegmentQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1_20270101::model::CompositeEventsPlatformQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "context")]
    pub context: Option<crate::datadogV1_20270101::model::EventsContext>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::ListSegmentQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "segment_query")]
    pub segment_query: crate::datadogV1_20270101::model::ListSegmentQuerySegmentQuery,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ListSegmentQuery {
    pub fn new(base_query: crate::datadogV1_20270101::model::CompositeEventsPlatformQuery, data_source: crate::datadogV1_20270101::model::ListSegmentQueryDataSource, name: String, segment_query: crate::datadogV1_20270101::model::ListSegmentQuerySegmentQuery) -> ListSegmentQuery {
        ListSegmentQuery {
            base_query,
            context: None,
            data_source,
            name,
            segment_query,
            _unparsed: false,
        }
    }
    
    pub fn context(mut self, value: crate::datadogV1_20270101::model::EventsContext) -> Self {
        self.context = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ListSegmentQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListSegmentQueryVisitor;
        impl<'a> Visitor<'a> for ListSegmentQueryVisitor {
            type Value = ListSegmentQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<crate::datadogV1_20270101::model::CompositeEventsPlatformQuery> = None;
                let mut context: Option<crate::datadogV1_20270101::model::EventsContext> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::ListSegmentQueryDataSource> = None;
                let mut name: Option<String> = None;
                let mut segment_query: Option<crate::datadogV1_20270101::model::ListSegmentQuerySegmentQuery> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::ListSegmentQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let segment_query = segment_query.ok_or_else(|| M::Error::missing_field("segment_query"))?;

                let content = ListSegmentQuery {
                    base_query,
                    context,
                    data_source,
                    name,
                    segment_query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListSegmentQueryVisitor)
    }
}

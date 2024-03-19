// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual scalar events query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsScalarQuery {
    /// The instructions for what to compute for this query.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::EventsCompute,
    /// A data source that is powered by the Events Platform.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::EventsDataSource,
    /// The list of facets on which to split results.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::EventsGroupBy>>,
    /// The indexes in which to search.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Configuration of the search/filter for an events query.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::EventsSearch>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsScalarQuery {
    pub fn new(
        compute: crate::datadogV2::model::EventsCompute,
        data_source: crate::datadogV2::model::EventsDataSource,
    ) -> EventsScalarQuery {
        EventsScalarQuery {
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name: None,
            search: None,
            _unparsed: false,
        }
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::EventsGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn search(mut self, value: crate::datadogV2::model::EventsSearch) -> Self {
        self.search = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EventsScalarQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsScalarQueryVisitor;
        impl<'a> Visitor<'a> for EventsScalarQueryVisitor {
            type Value = EventsScalarQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::EventsCompute> = None;
                let mut data_source: Option<crate::datadogV2::model::EventsDataSource> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::EventsGroupBy>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV2::model::EventsSearch> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::EventsDataSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;

                let content = EventsScalarQuery {
                    compute,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsScalarQueryVisitor)
    }
}

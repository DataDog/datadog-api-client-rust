// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query configuration for Sankey network widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyNetworkQuery {
    /// Compute aggregation for network queries.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1::model::SankeyNetworkQueryCompute>,
    /// Network data source type.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::SankeyNetworkDataSource,
    /// Fields to group by.
    #[serde(rename = "group_by")]
    pub group_by: Vec<String>,
    /// Maximum number of results.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Sankey mode for network queries.
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV1::model::SankeyNetworkQueryMode>,
    /// Query string for filtering network data.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Whether to exclude missing values.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Sort configuration for network queries.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::SankeyNetworkQuerySort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyNetworkQuery {
    pub fn new(
        data_source: crate::datadogV1::model::SankeyNetworkDataSource,
        group_by: Vec<String>,
        limit: i64,
        query_string: String,
    ) -> SankeyNetworkQuery {
        SankeyNetworkQuery {
            compute: None,
            data_source,
            group_by,
            limit,
            mode: None,
            query_string,
            should_exclude_missing: None,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: crate::datadogV1::model::SankeyNetworkQueryCompute) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn mode(mut self, value: crate::datadogV1::model::SankeyNetworkQueryMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn should_exclude_missing(mut self, value: bool) -> Self {
        self.should_exclude_missing = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::SankeyNetworkQuerySort) -> Self {
        self.sort = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SankeyNetworkQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyNetworkQueryVisitor;
        impl<'a> Visitor<'a> for SankeyNetworkQueryVisitor {
            type Value = SankeyNetworkQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::SankeyNetworkQueryCompute> = None;
                let mut data_source: Option<crate::datadogV1::model::SankeyNetworkDataSource> =
                    None;
                let mut group_by: Option<Vec<String>> = None;
                let mut limit: Option<i64> = None;
                let mut mode: Option<crate::datadogV1::model::SankeyNetworkQueryMode> = None;
                let mut query_string: Option<String> = None;
                let mut should_exclude_missing: Option<bool> = None;
                let mut sort: Option<crate::datadogV1::model::SankeyNetworkQuerySort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV1::model::SankeyNetworkDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1::model::SankeyNetworkQueryMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_exclude_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            should_exclude_missing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = SankeyNetworkQuery {
                    compute,
                    data_source,
                    group_by,
                    limit,
                    mode,
                    query_string,
                    should_exclude_missing,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyNetworkQueryVisitor)
    }
}

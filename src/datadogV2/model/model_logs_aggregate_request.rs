// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object sent with the request to retrieve a list of logs from your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsAggregateRequest {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV2::model::LogsCompute>>,
    /// The search and filter query settings
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsQueryFilter>,
    /// The rules for the group by
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::LogsGroupBy>>,
    /// Global query options that are used during the query.
    /// Note: you should supply either timezone or time offset, but not both. Otherwise, the query will fail.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::LogsQueryOptions>,
    /// Paging settings
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LogsAggregateRequestPage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsAggregateRequest {
    pub fn new() -> LogsAggregateRequest {
        LogsAggregateRequest {
            compute: None,
            filter: None,
            group_by: None,
            options: None,
            page: None,
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: Vec<crate::datadogV2::model::LogsCompute>) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::LogsQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::LogsGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::LogsQueryOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::LogsAggregateRequestPage) -> Self {
        self.page = Some(value);
        self
    }
}

impl Default for LogsAggregateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsAggregateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsAggregateRequestVisitor;
        impl<'a> Visitor<'a> for LogsAggregateRequestVisitor {
            type Value = LogsAggregateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<Vec<crate::datadogV2::model::LogsCompute>> = None;
                let mut filter: Option<crate::datadogV2::model::LogsQueryFilter> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::LogsGroupBy>> = None;
                let mut options: Option<crate::datadogV2::model::LogsQueryOptions> = None;
                let mut page: Option<crate::datadogV2::model::LogsAggregateRequestPage> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsAggregateRequest {
                    compute,
                    filter,
                    group_by,
                    options,
                    page,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsAggregateRequestVisitor)
    }
}

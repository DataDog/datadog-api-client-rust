// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to send with the request to retrieve a list of logs from your Organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsListRequest {
    /// The log index on which the request is performed. For multi-index organizations,
    /// the default is all live indexes. Historical indexes of rehydrated logs must be specified.
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// Number of logs return in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The search query - following the log search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Time-ascending `asc` or time-descending `desc` results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::LogsSort>,
    /// Hash identifier of the first log to return in the list, available in a log `id` attribute.
    /// This parameter is used for the pagination feature.
    ///
    /// **Note**: This parameter is ignored if the corresponding log
    /// is out of the scope of the specified time window.
    #[serde(rename = "startAt")]
    pub start_at: Option<String>,
    /// Timeframe to retrieve the log from.
    #[serde(rename = "time")]
    pub time: crate::datadogV1::model::LogsListRequestTime,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsListRequest {
    pub fn new(time: crate::datadogV1::model::LogsListRequestTime) -> LogsListRequest {
        LogsListRequest {
            index: None,
            limit: None,
            query: None,
            sort: None,
            start_at: None,
            time,
            _unparsed: false,
        }
    }

    pub fn index(&mut self, value: String) -> &mut Self {
        self.index = Some(value);
        self
    }

    pub fn limit(&mut self, value: i32) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn sort(&mut self, value: crate::datadogV1::model::LogsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }

    pub fn start_at(&mut self, value: String) -> &mut Self {
        self.start_at = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsListRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsListRequestVisitor;
        impl<'a> Visitor<'a> for LogsListRequestVisitor {
            type Value = LogsListRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut index: Option<String> = None;
                let mut limit: Option<i32> = None;
                let mut query: Option<String> = None;
                let mut sort: Option<crate::datadogV1::model::LogsSort> = None;
                let mut start_at: Option<String> = None;
                let mut time: Option<crate::datadogV1::model::LogsListRequestTime> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1::model::LogsSort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "startAt" => {
                            if v.is_null() {
                                continue;
                            }
                            start_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let time = time.ok_or_else(|| M::Error::missing_field("time"))?;

                let content = LogsListRequest {
                    index,
                    limit,
                    query,
                    sort,
                    start_at,
                    time,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsListRequestVisitor)
    }
}

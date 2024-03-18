// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response Object that includes your query and the list of metrics retrieved.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsQueryResponse {
    /// Message indicating the errors if status is not `ok`.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_date")]
    pub from_date: Option<i64>,
    /// List of tag keys on which to group.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Message indicating `success` if status is `ok`.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Query string
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Type of response.
    #[serde(rename = "res_type")]
    pub res_type: Option<String>,
    /// List of timeseries queried.
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV1::model::MetricsQueryMetadata>>,
    /// Status of the query.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_date")]
    pub to_date: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricsQueryResponse {
    pub fn new() -> MetricsQueryResponse {
        MetricsQueryResponse {
            error: None,
            from_date: None,
            group_by: None,
            message: None,
            query: None,
            res_type: None,
            series: None,
            status: None,
            to_date: None,
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
        self
    }

    pub fn from_date(mut self, value: i64) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn res_type(mut self, value: String) -> Self {
        self.res_type = Some(value);
        self
    }

    pub fn series(mut self, value: Vec<crate::datadogV1::model::MetricsQueryMetadata>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn to_date(mut self, value: i64) -> Self {
        self.to_date = Some(value);
        self
    }
}

impl Default for MetricsQueryResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricsQueryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsQueryResponseVisitor;
        impl<'a> Visitor<'a> for MetricsQueryResponseVisitor {
            type Value = MetricsQueryResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<String> = None;
                let mut from_date: Option<i64> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut message: Option<String> = None;
                let mut query: Option<String> = None;
                let mut res_type: Option<String> = None;
                let mut series: Option<Vec<crate::datadogV1::model::MetricsQueryMetadata>> = None;
                let mut status: Option<String> = None;
                let mut to_date: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from_date" => {
                            if v.is_null() {
                                continue;
                            }
                            from_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "res_type" => {
                            if v.is_null() {
                                continue;
                            }
                            res_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "series" => {
                            if v.is_null() {
                                continue;
                            }
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_date" => {
                            if v.is_null() {
                                continue;
                            }
                            to_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricsQueryResponse {
                    error,
                    from_date,
                    group_by,
                    message,
                    query,
                    res_type,
                    series,
                    status,
                    to_date,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsQueryResponseVisitor)
    }
}

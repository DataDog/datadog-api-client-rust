// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A `metric` based SLO history response.
///
/// This is not included in responses for `monitor` based SLOs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryMetrics {
    /// A representation of `metric` based SLO timeseries for the provided queries.
    /// This is the same response type from `batch_query` endpoint.
    #[serde(rename = "denominator")]
    pub denominator: crate::datadogV1::model::SLOHistoryMetricsSeries,
    /// The aggregated query interval for the series data. It's implicit based on the query time window.
    #[serde(rename = "interval")]
    pub interval: i64,
    /// Optional message if there are specific query issues/warnings.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// A representation of `metric` based SLO timeseries for the provided queries.
    /// This is the same response type from `batch_query` endpoint.
    #[serde(rename = "numerator")]
    pub numerator: crate::datadogV1::model::SLOHistoryMetricsSeries,
    /// The combined numerator and denominator query CSV.
    #[serde(rename = "query")]
    pub query: String,
    /// The series result type. This mimics `batch_query` response type.
    #[serde(rename = "res_type")]
    pub res_type: String,
    /// The series response version type. This mimics `batch_query` response type.
    #[serde(rename = "resp_version")]
    pub resp_version: i64,
    /// An array of query timestamps in EPOCH milliseconds.
    #[serde(rename = "times")]
    pub times: Vec<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryMetrics {
    pub fn new(
        denominator: crate::datadogV1::model::SLOHistoryMetricsSeries,
        interval: i64,
        numerator: crate::datadogV1::model::SLOHistoryMetricsSeries,
        query: String,
        res_type: String,
        resp_version: i64,
        times: Vec<f64>,
    ) -> SLOHistoryMetrics {
        SLOHistoryMetrics {
            denominator,
            interval,
            message: None,
            numerator,
            query,
            res_type,
            resp_version,
            times,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SLOHistoryMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryMetricsVisitor;
        impl<'a> Visitor<'a> for SLOHistoryMetricsVisitor {
            type Value = SLOHistoryMetrics;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denominator: Option<crate::datadogV1::model::SLOHistoryMetricsSeries> =
                    None;
                let mut interval: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut numerator: Option<crate::datadogV1::model::SLOHistoryMetricsSeries> = None;
                let mut query: Option<String> = None;
                let mut res_type: Option<String> = None;
                let mut resp_version: Option<i64> = None;
                let mut times: Option<Vec<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denominator" => {
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "res_type" => {
                            res_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resp_version" => {
                            resp_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times" => {
                            times = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let denominator =
                    denominator.ok_or_else(|| M::Error::missing_field("denominator"))?;
                let interval = interval.ok_or_else(|| M::Error::missing_field("interval"))?;
                let numerator = numerator.ok_or_else(|| M::Error::missing_field("numerator"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let res_type = res_type.ok_or_else(|| M::Error::missing_field("res_type"))?;
                let resp_version =
                    resp_version.ok_or_else(|| M::Error::missing_field("resp_version"))?;
                let times = times.ok_or_else(|| M::Error::missing_field("times"))?;

                let content = SLOHistoryMetrics {
                    denominator,
                    interval,
                    message,
                    numerator,
                    query,
                    res_type,
                    resp_version,
                    times,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryMetricsVisitor)
    }
}

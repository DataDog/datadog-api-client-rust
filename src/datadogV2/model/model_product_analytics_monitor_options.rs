// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options of a Product Analytics monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorOptions {
    /// UUID of the chart the monitor was created from.
    #[serde(rename = "chart_uuid")]
    pub chart_uuid: Option<String>,
    /// Metric of the chart that the monitor evaluates.
    #[serde(rename = "metric_selector")]
    pub metric_selector: Option<String>,
    /// Index of the chart query that the monitor evaluates.
    #[serde(rename = "query_index")]
    pub query_index: Option<i64>,
    /// Number of minutes after the last notification before the monitor notifies again on the current status.
    /// `0` means it never notifies again.
    #[serde(rename = "renotify_interval")]
    pub renotify_interval: Option<i64>,
    /// Alert thresholds of a Product Analytics monitor.
    /// Keep `critical` in sync with the comparison in the monitor query; the query is not derived from these values.
    #[serde(rename = "thresholds")]
    pub thresholds: Option<crate::datadogV2::model::ProductAnalyticsMonitorThresholds>,
    /// Time frame of the chart that the monitor evaluates.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<String>,
    /// Query variables referenced by the monitor query.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorOptions {
    pub fn new() -> ProductAnalyticsMonitorOptions {
        ProductAnalyticsMonitorOptions {
            chart_uuid: None,
            metric_selector: None,
            query_index: None,
            renotify_interval: None,
            thresholds: None,
            timeframe: None,
            variables: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chart_uuid(mut self, value: String) -> Self {
        self.chart_uuid = Some(value);
        self
    }

    pub fn metric_selector(mut self, value: String) -> Self {
        self.metric_selector = Some(value);
        self
    }

    pub fn query_index(mut self, value: i64) -> Self {
        self.query_index = Some(value);
        self
    }

    pub fn renotify_interval(mut self, value: i64) -> Self {
        self.renotify_interval = Some(value);
        self
    }

    pub fn thresholds(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsMonitorThresholds,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    pub fn timeframe(mut self, value: String) -> Self {
        self.timeframe = Some(value);
        self
    }

    pub fn variables(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.variables = Some(value);
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

impl Default for ProductAnalyticsMonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorOptionsVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorOptionsVisitor {
            type Value = ProductAnalyticsMonitorOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chart_uuid: Option<String> = None;
                let mut metric_selector: Option<String> = None;
                let mut query_index: Option<i64> = None;
                let mut renotify_interval: Option<i64> = None;
                let mut thresholds: Option<
                    crate::datadogV2::model::ProductAnalyticsMonitorThresholds,
                > = None;
                let mut timeframe: Option<String> = None;
                let mut variables: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chart_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            chart_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_selector" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_selector =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_index" => {
                            if v.is_null() {
                                continue;
                            }
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "renotify_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            renotify_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thresholds" => {
                            if v.is_null() {
                                continue;
                            }
                            thresholds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsMonitorOptions {
                    chart_uuid,
                    metric_selector,
                    query_index,
                    renotify_interval,
                    thresholds,
                    timeframe,
                    variables,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorOptionsVisitor)
    }
}

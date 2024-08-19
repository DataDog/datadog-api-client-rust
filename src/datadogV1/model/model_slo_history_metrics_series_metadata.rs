// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryMetricsSeriesMetadata {
    /// Query aggregator function.
    #[serde(rename = "aggr")]
    pub aggr: Option<String>,
    /// Query expression.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Query metric used.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Query index from original combined query.
    #[serde(rename = "query_index")]
    pub query_index: Option<i64>,
    /// Query scope.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// An array of metric units that contains up to two unit objects.
    /// For example, bytes represents one unit object and bytes per second represents two unit objects.
    /// If a metric query only has one unit object, the second array element is null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit:
        Option<Option<Vec<Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadataUnit>>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryMetricsSeriesMetadata {
    pub fn new() -> SLOHistoryMetricsSeriesMetadata {
        SLOHistoryMetricsSeriesMetadata {
            aggr: None,
            expression: None,
            metric: None,
            query_index: None,
            scope: None,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggr(mut self, value: String) -> Self {
        self.aggr = Some(value);
        self
    }

    pub fn expression(mut self, value: String) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn query_index(mut self, value: i64) -> Self {
        self.query_index = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn unit(
        mut self,
        value: Option<Vec<Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadataUnit>>>,
    ) -> Self {
        self.unit = Some(value);
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

impl Default for SLOHistoryMetricsSeriesMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOHistoryMetricsSeriesMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryMetricsSeriesMetadataVisitor;
        impl<'a> Visitor<'a> for SLOHistoryMetricsSeriesMetadataVisitor {
            type Value = SLOHistoryMetricsSeriesMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggr: Option<String> = None;
                let mut expression: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut query_index: Option<i64> = None;
                let mut scope: Option<String> = None;
                let mut unit: Option<
                    Option<
                        Vec<Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadataUnit>>,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggr" => {
                            if v.is_null() {
                                continue;
                            }
                            aggr = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            if v.is_null() {
                                continue;
                            }
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_index" => {
                            if v.is_null() {
                                continue;
                            }
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SLOHistoryMetricsSeriesMetadata {
                    aggr,
                    expression,
                    metric,
                    query_index,
                    scope,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryMetricsSeriesMetadataVisitor)
    }
}

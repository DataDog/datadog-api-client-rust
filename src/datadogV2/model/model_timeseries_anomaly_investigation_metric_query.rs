// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metrics query referenced by a formula.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationMetricQuery {
    /// Optional scalar aggregator accepted for request compatibility. This field is ignored for timeseries queries.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<String>,
    /// Optional organization UUID used for a cross-organization query. Each query accepts at most one UUID; use separate queries for separate organizations. Influential-tag analysis is currently unsupported for cross-organization queries, but anomaly detection still runs.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Data source for an anomaly investigation query.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::TimeseriesAnomalyInvestigationDataSource,
    /// Name used to reference this query from formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Datadog metrics query expression.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationMetricQuery {
    pub fn new(
        data_source: crate::datadogV2::model::TimeseriesAnomalyInvestigationDataSource,
        name: String,
        query: String,
    ) -> TimeseriesAnomalyInvestigationMetricQuery {
        TimeseriesAnomalyInvestigationMetricQuery {
            aggregator: None,
            cross_org_uuids: None,
            data_source,
            name,
            query,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregator(mut self, value: String) -> Self {
        self.aggregator = Some(value);
        self
    }

    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationMetricQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationMetricQueryVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationMetricQueryVisitor {
            type Value = TimeseriesAnomalyInvestigationMetricQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<String> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationDataSource,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cross_org_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_org_uuids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = TimeseriesAnomalyInvestigationMetricQuery {
                    aggregator,
                    cross_org_uuids,
                    data_source,
                    name,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationMetricQueryVisitor)
    }
}

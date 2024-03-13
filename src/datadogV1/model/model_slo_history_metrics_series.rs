// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A representation of `metric` based SLO time series for the provided queries.
/// This is the same response type from `batch_query` endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryMetricsSeries {
    /// Count of submitted metrics.
    #[serde(rename = "count")]
    pub count: i64,
    /// Query metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadata>,
    /// Total sum of the query.
    #[serde(rename = "sum")]
    pub sum: f64,
    /// The query values for each metric.
    #[serde(rename = "values")]
    pub values: Vec<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryMetricsSeries {
    pub fn new(count: i64, sum: f64, values: Vec<f64>) -> SLOHistoryMetricsSeries {
        SLOHistoryMetricsSeries {
            count,
            metadata: None,
            sum,
            values,
            _unparsed: false,
        }
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV1::model::SLOHistoryMetricsSeriesMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOHistoryMetricsSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryMetricsSeriesVisitor;
        impl<'a> Visitor<'a> for SLOHistoryMetricsSeriesVisitor {
            type Value = SLOHistoryMetricsSeries;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut metadata: Option<crate::datadogV1::model::SLOHistoryMetricsSeriesMetadata> =
                    None;
                let mut sum: Option<f64> = None;
                let mut values: Option<Vec<f64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sum" => {
                            sum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let sum = sum.ok_or_else(|| M::Error::missing_field("sum"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = SLOHistoryMetricsSeries {
                    count,
                    metadata,
                    sum,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryMetricsSeriesVisitor)
    }
}

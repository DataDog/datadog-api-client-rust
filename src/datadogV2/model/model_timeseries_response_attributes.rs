// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a timeseries response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesResponseAttributes {
    /// Array of response series. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV2::model::TimeseriesResponseSeries>>,
    /// Array of times, 1-1 match with individual values arrays.
    #[serde(rename = "times")]
    pub times: Option<Vec<i64>>,
    /// Array of value-arrays. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "values")]
    pub values: Option<Vec<Vec<Option<f64>>>>,
}

impl TimeseriesResponseAttributes {
    pub fn new() -> TimeseriesResponseAttributes {
        TimeseriesResponseAttributes {
            series: None,
            times: None,
            values: None,
        }
    }

    pub fn series(mut self, value: Vec<crate::datadogV2::model::TimeseriesResponseSeries>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn times(mut self, value: Vec<i64>) -> Self {
        self.times = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Vec<Option<f64>>>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for TimeseriesResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

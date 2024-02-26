// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A timeseries point.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppAggregateBucketValueTimeseriesPoint {
    /// The time value for this point.
    #[serde(rename = "time")]
    pub time: Option<String>,
    /// The value for this point.
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

impl CIAppAggregateBucketValueTimeseriesPoint {
    pub fn new() -> CIAppAggregateBucketValueTimeseriesPoint {
        CIAppAggregateBucketValueTimeseriesPoint {
            time: None,
            value: None,
        }
    }

    pub fn time(&mut self, value: String) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn value(&mut self, value: f64) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for CIAppAggregateBucketValueTimeseriesPoint {
    fn default() -> Self {
        Self::new()
    }
}

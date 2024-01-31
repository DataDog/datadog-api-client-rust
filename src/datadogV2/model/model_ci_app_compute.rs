// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A compute rule to compute metrics or timeseries.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCompute {
    /// An aggregation function.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::CIAppAggregationFunction,
    /// The time buckets' size (only used for type=timeseries)
    /// Defaults to a resolution of 150 points.
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// The metric to use.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The type of compute.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CIAppComputeType>,
}

impl CIAppCompute {
    pub fn new(aggregation: crate::datadogV2::model::CIAppAggregationFunction) -> CIAppCompute {
        CIAppCompute {
            aggregation,
            interval: None,
            metric: None,
            type_: None,
        }
    }

    pub fn with_interval(&mut self, value: String) -> &mut Self {
        self.interval = Some(value);
        self
    }

    pub fn with_metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::CIAppComputeType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Defines the metric and aggregation used as the sort value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitConfigSortCompute {
    /// How to aggregate the sort metric for the purposes of ordering.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// The metric to use for sorting graphs.
    #[serde(rename = "metric")]
    pub metric: String,
}

impl SplitConfigSortCompute {
    pub fn new(aggregation: String, metric: String) -> SplitConfigSortCompute {
        SplitConfigSortCompute {
            aggregation,
            metric,
        }
    }
}

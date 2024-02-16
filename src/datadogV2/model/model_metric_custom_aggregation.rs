// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A time and space aggregation combination for use in query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCustomAggregation {
    /// A space aggregation for use in query.
    #[serde(rename = "space")]
    pub space: crate::datadogV2::model::MetricCustomSpaceAggregation,
    /// A time aggregation for use in query.
    #[serde(rename = "time")]
    pub time: crate::datadogV2::model::MetricCustomTimeAggregation,
}

impl MetricCustomAggregation {
    pub fn new(
        space: crate::datadogV2::model::MetricCustomSpaceAggregation,
        time: crate::datadogV2::model::MetricCustomTimeAggregation,
    ) -> MetricCustomAggregation {
        MetricCustomAggregation { space, time }
    }
}

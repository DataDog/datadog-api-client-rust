// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing the Datadog span-based metric to create.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricCreateAttributes {
    /// The compute rule to compute the span-based metric.
    #[serde(rename = "compute")]
    pub compute: Box<crate::datadogV2::model::SpansMetricCompute>,
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::SpansMetricFilter>>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::SpansMetricGroupBy>>,
}

impl SpansMetricCreateAttributes {
    pub fn new(
        compute: Box<crate::datadogV2::model::SpansMetricCompute>,
    ) -> SpansMetricCreateAttributes {
        SpansMetricCreateAttributes {
            compute,
            filter: None,
            group_by: None,
        }
    }
}

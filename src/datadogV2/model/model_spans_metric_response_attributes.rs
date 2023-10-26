// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricResponseAttributes {
    /// The compute rule to compute the span-based metric.
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: Option<Box<crate::datadogV2::model::SpansMetricResponseCompute>>,
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::datadogV2::model::SpansMetricResponseFilter>>,
    /// The rules for the group by.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<crate::datadogV2::model::SpansMetricResponseGroupBy>>,
}

impl SpansMetricResponseAttributes {
    /// The object describing a Datadog span-based metric.
    pub fn new() -> SpansMetricResponseAttributes {
        SpansMetricResponseAttributes {
            compute: None,
            filter: None,
            group_by: None,
        }
    }
}

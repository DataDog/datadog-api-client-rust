// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a Datadog span-based metric.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricResponseAttributes {
    /// The compute rule to compute the span-based metric.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV2::model::SpansMetricResponseCompute>,
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SpansMetricResponseFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::SpansMetricResponseGroupBy>>,
}

impl SpansMetricResponseAttributes {
    pub fn new() -> SpansMetricResponseAttributes {
        SpansMetricResponseAttributes {
            compute: None,
            filter: None,
            group_by: None,
        }
    }

    pub fn compute(
        &mut self,
        value: crate::datadogV2::model::SpansMetricResponseCompute,
    ) -> &mut Self {
        self.compute = Some(value);
        self
    }

    pub fn filter(
        &mut self,
        value: crate::datadogV2::model::SpansMetricResponseFilter,
    ) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(
        &mut self,
        value: Vec<crate::datadogV2::model::SpansMetricResponseGroupBy>,
    ) -> &mut Self {
        self.group_by = Some(value);
        self
    }
}

impl Default for SpansMetricResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

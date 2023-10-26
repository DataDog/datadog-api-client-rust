// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metric origin information.
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Box<crate::datadogV2::model::MetricOrigin>>,
}

impl MetricMetadata {
    /// Metadata for the metric.
    pub fn new() -> MetricMetadata {
        MetricMetadata { origin: None }
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The span-based metric properties.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricResponseData {
    /// The object describing a Datadog span-based metric.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SpansMetricResponseAttributes>>,
    /// The name of the span-based metric.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of resource. The value should always be spans_metrics.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansMetricType>,
}

impl SpansMetricResponseData {
    pub fn new() -> SpansMetricResponseData {
        SpansMetricResponseData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}

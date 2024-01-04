// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a metric cardinality estimate.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricEstimate {
    /// Object containing the definition of a metric estimate attribute.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MetricEstimateAttributes>>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The metric estimate resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricEstimateResourceType>,
}

impl MetricEstimate {
    pub fn new() -> MetricEstimate {
        MetricEstimate {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
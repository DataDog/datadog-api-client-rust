// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single metric tag configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The metric resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricType>,
}

impl Metric {
    pub fn new() -> Metric {
        Metric {
            id: None,
            type_: None,
        }
    }
}
impl Default for Metric {
    fn default() -> Self {
        Self::new()
    }
}

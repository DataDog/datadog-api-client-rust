// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single metric's distinct volume.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDistinctVolume {
    /// Object containing the definition of a metric's distinct volume.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MetricDistinctVolumeAttributes>>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The metric distinct volume type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricDistinctVolumeType>,
}

impl MetricDistinctVolume {
    pub fn new() -> MetricDistinctVolume {
        MetricDistinctVolume {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for MetricDistinctVolume {
    fn default() -> Self {
        Self::new()
    }
}

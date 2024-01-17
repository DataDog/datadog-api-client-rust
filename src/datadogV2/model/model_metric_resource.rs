// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metric resource.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricResource {
    /// The name of the resource.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of the resource.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl MetricResource {
    pub fn new() -> MetricResource {
        MetricResource {
            name: None,
            type_: None,
        }
    }
}

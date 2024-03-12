// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of definitions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinitionRequests {
    /// Updated host map.
    #[serde(rename = "fill")]
    pub fill: Option<crate::datadogV1::model::HostMapRequest>,
    /// Updated host map.
    #[serde(rename = "size")]
    pub size: Option<crate::datadogV1::model::HostMapRequest>,
}

impl HostMapWidgetDefinitionRequests {
    pub fn new() -> HostMapWidgetDefinitionRequests {
        HostMapWidgetDefinitionRequests {
            fill: None,
            size: None,
        }
    }

    pub fn fill(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.fill = Some(value);
        self
    }

    pub fn size(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.size = Some(value);
        self
    }
}

impl Default for HostMapWidgetDefinitionRequests {
    fn default() -> Self {
        Self::new()
    }
}

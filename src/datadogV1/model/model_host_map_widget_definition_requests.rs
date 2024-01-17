// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of definitions.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinitionRequests {
    /// Updated host map.
    #[serde(rename = "fill")]
    pub fill: Option<Box<crate::datadogV1::model::HostMapRequest>>,
    /// Updated host map.
    #[serde(rename = "size")]
    pub size: Option<Box<crate::datadogV1::model::HostMapRequest>>,
}

impl HostMapWidgetDefinitionRequests {
    pub fn new() -> HostMapWidgetDefinitionRequests {
        HostMapWidgetDefinitionRequests {
            fill: None,
            size: None,
        }
    }
}

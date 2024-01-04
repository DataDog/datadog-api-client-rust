// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all event attributes and their associated values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventAttributes {
    /// JSON object of attributes from CI Visibility pipeline events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Pipeline execution level.
    #[serde(rename = "ci_level")]
    pub ci_level: Option<crate::datadogV2::model::CIAppPipelineLevel>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl CIAppPipelineEventAttributes {
    pub fn new() -> CIAppPipelineEventAttributes {
        CIAppPipelineEventAttributes {
            attributes: None,
            ci_level: None,
            tags: None,
        }
    }
}

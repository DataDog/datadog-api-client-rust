// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all event attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventAttributes {
    /// JSON object of attributes from CI Visibility pipeline events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
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

    pub fn attributes(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn ci_level(&mut self, value: crate::datadogV2::model::CIAppPipelineLevel) -> &mut Self {
        self.ci_level = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl Default for CIAppPipelineEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

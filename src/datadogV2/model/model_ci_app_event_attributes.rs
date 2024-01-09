// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all event attributes and their associated values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppEventAttributes {
    /// JSON object of attributes from CI Visibility test events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Test run level.
    #[serde(rename = "test_level")]
    pub test_level: Option<crate::datadogV2::model::CIAppTestLevel>,
}

impl CIAppEventAttributes {
    pub fn new() -> CIAppEventAttributes {
        CIAppEventAttributes {
            attributes: None,
            tags: None,
            test_level: None,
        }
    }
}
impl Default for CIAppEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

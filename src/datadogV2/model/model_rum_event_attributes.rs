// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all event attributes and their associated values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMEventAttributes {
    /// JSON object of attributes from RUM events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The name of the application or service generating RUM events.
    /// It is used to switch from RUM to APM, so make sure you define the same
    /// value when you use both products.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp of your event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

impl RUMEventAttributes {
    pub fn new() -> RUMEventAttributes {
        RUMEventAttributes {
            attributes: None,
            service: None,
            tags: None,
            timestamp: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_timestamp(&mut self, value: String) -> &mut Self {
        self.timestamp = Some(value);
        self
    }
}
impl Default for RUMEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

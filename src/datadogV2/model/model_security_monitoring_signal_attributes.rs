// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing all signal attributes and their
/// associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalAttributes {
    /// A JSON object of attributes in the security signal.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The message in the security signal defined by the rule that generated the signal.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// An array of tags associated with the security signal.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The timestamp of the security signal.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}

impl SecurityMonitoringSignalAttributes {
    pub fn new() -> SecurityMonitoringSignalAttributes {
        SecurityMonitoringSignalAttributes {
            custom: None,
            message: None,
            tags: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
        }
    }

    pub fn custom(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SecurityMonitoringSignalAttributes {
    fn default() -> Self {
        Self::new()
    }
}

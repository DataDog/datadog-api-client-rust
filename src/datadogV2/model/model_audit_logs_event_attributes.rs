// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all event attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsEventAttributes {
    /// JSON object of attributes from Audit Logs events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Message of the event.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Name of the application or service generating Audit Logs events.
    /// This name is used to correlate Audit Logs to APM, so make sure you specify the same
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

impl AuditLogsEventAttributes {
    pub fn new() -> AuditLogsEventAttributes {
        AuditLogsEventAttributes {
            attributes: None,
            message: None,
            service: None,
            tags: None,
            timestamp: None,
        }
    }

    pub fn attributes(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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
}

impl Default for AuditLogsEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

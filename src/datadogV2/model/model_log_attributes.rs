// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON object containing all log attributes and their associated values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAttributes {
    /// JSON object of attributes from your log.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the machine from where the logs are being sent.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The message [reserved attribute](<https://docs.datadoghq.com/logs/log_collection/#reserved-attributes>)
    /// of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry.
    /// That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the application or service generating the log events.
    /// It is used to switch from Logs to APM, so make sure you define the same
    /// value when you use both products.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Status of the message associated with your log.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Array of tags associated with your log.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp of your log.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

impl LogAttributes {
    pub fn new() -> LogAttributes {
        LogAttributes {
            attributes: None,
            host: None,
            message: None,
            service: None,
            status: None,
            tags: None,
            timestamp: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }

    pub fn status(&mut self, value: String) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(&mut self, value: String) -> &mut Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for LogAttributes {
    fn default() -> Self {
        Self::new()
    }
}

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
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Name of the machine from where the logs are being sent.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The message [reserved attribute](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes)
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
}
impl Default for LogAttributes {
    fn default() -> Self {
        Self::new()
    }
}

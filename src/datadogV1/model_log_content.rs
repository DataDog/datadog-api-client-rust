// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogContent {
    /// JSON object of attributes from your log.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: map[string]interface{},
    /// Name of the machine from where the logs are being sent.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// The message [reserved attribute](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes)
of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry.
That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The name of the application or service generating the log events.
It is used to switch from Logs to APM, so make sure you define the same
value when you use both products.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Array of tags associated with your log.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Timestamp of your log.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: String,
}


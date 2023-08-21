// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogItem {
    /// The integration name associated with your log: the technology from which the log originated.
When it matches an integration name, Datadog automatically installs the corresponding parsers and facets.
See [reserved attributes](https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes).
    #[serde(rename = "ddsource", skip_serializing_if = "Option::is_none")]
    pub ddsource: String,
    /// Tags associated with your logs.
    #[serde(rename = "ddtags", skip_serializing_if = "Option::is_none")]
    pub ddtags: String,
    /// The name of the originating host of the log.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: String,
    /// The message [reserved attribute](https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes)
of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry.
That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The name of the application or service generating the log events.
It is used to switch from Logs to APM, so make sure you define the same value when you use both products.
See [reserved attributes](https://docs.datadoghq.com/logs/log_configuration/attributes_naming_convention/#reserved-attributes).
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
}


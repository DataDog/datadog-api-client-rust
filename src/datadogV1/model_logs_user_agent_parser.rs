// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsUserAgentParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Define if the source attribute is URL encoded or not.
    #[serde(rename = "is_encoded", skip_serializing_if = "Option::is_none")]
    pub is_encoded: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Array of source attributes.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// Type of logs User-Agent parser.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsUserAgentParserType,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGrokParser {
    /// Set of rules for the grok parser.
    #[serde(rename = "grok")]
    pub grok: LogsGrokParserRules,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// List of sample logs to test this grok parser.
    #[serde(rename = "samples", skip_serializing_if = "Option::is_none")]
    pub samples: Vec<String>,
    /// Name of the log attribute to parse.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: String,
    /// Type of logs grok parser.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsGrokParserType,
}


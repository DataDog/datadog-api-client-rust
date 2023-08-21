// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsURLParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Normalize the ending slashes or not.
    #[serde(rename = "normalize_ending_slashes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub normalize_ending_slashes: Option<Bool>,
    /// Array of source attributes.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// Type of logs URL parser.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsURLParserType,
}


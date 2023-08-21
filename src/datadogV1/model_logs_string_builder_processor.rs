// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsStringBuilderProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// If true, it replaces all missing attributes of `template` by an empty string.
If `false` (default), skips the operation for missing attributes.
    #[serde(rename = "is_replace_missing", skip_serializing_if = "Option::is_none")]
    pub is_replace_missing: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The name of the attribute that contains the result of the template.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// A formula with one or more attributes and raw text.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: String,
    /// Type of logs string builder processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsStringBuilderProcessorType,
}


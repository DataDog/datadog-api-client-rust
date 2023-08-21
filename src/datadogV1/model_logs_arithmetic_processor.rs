// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArithmeticProcessor {
    /// Arithmetic operation between one or more log attributes.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: String,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// If `true`, it replaces all missing attributes of expression by `0`, `false`
skip the operation if an attribute is missing.
    #[serde(rename = "is_replace_missing", skip_serializing_if = "Option::is_none")]
    pub is_replace_missing: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Name of the attribute that contains the result of the arithmetic operation.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// Type of logs arithmetic processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsArithmeticProcessorType,
}


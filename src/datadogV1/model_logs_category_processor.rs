// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsCategoryProcessor {
    /// Array of filters to match or not a log and their
corresponding `name` to assign a custom value to the log.
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Vec<LogsCategoryProcessorCategory>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Name of the target attribute which value is defined by the matching category.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// Type of logs category processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsCategoryProcessorType,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipelineProcessor {
    /// Filter for logs.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: LogsFilter,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Vec<LogsProcessor>,
    /// Type of logs pipeline processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsPipelineProcessorType,
}


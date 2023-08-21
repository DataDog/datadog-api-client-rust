// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceTableLogsLookupProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the Reference Table for the source attribute and their associated target attribute values.
    #[serde(rename = "lookup_enrichment_table", skip_serializing_if = "Option::is_none")]
    pub lookup_enrichment_table: String,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Source attribute used to perform the lookup.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: String,
    /// Name of the attribute that contains the corresponding value in the mapping list.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// Type of logs lookup processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsLookupProcessorType,
}


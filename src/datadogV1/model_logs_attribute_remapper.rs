// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAttributeRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: bool,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Override or not the target element if already set,
    #[serde(rename = "override_on_conflict", skip_serializing_if = "Option::is_none")]
    pub override_on_conflict: bool,
    /// Remove or preserve the remapped source element.
    #[serde(rename = "preserve_source", skip_serializing_if = "Option::is_none")]
    pub preserve_source: bool,
    /// Defines if the sources are from log `attribute` or `tag`.
    #[serde(rename = "source_type", skip_serializing_if = "Option::is_none")]
    pub source_type: String,
    /// Array of source attributes.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Vec<String>,
    /// Final attribute or tag name to remap the sources to.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: String,
    /// If the `target_type` of the remapper is `attribute`, try to cast the value to a new specific type.
If the cast is not possible, the original type is kept. `string`, `integer`, or `double` are the possible types.
If the `target_type` is `tag`, this parameter may not be specified.
    #[serde(rename = "target_format", skip_serializing_if = "Option::is_none")]
    pub target_format: TargetFormatType,
    /// Defines if the final attribute or tag name is from log `attribute` or `tag`.
    #[serde(rename = "target_type", skip_serializing_if = "Option::is_none")]
    pub target_type: String,
    /// Type of logs attribute remapper.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogsAttributeRemapperType,
}


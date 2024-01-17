// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The remapper processor remaps any source attribute(s) or tag to another target attribute or tag.
/// Constraints on the tag/attribute name are explained in the [Tag Best Practice documentation](https://docs.datadoghq.com/logs/guide/log-parsing-best-practice).
/// Some additional constraints are applied as `:` or `,` are not allowed in the target tag/attribute name.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAttributeRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Override or not the target element if already set,
    #[serde(rename = "override_on_conflict")]
    pub override_on_conflict: Option<bool>,
    /// Remove or preserve the remapped source element.
    #[serde(rename = "preserve_source")]
    pub preserve_source: Option<bool>,
    /// Defines if the sources are from log `attribute` or `tag`.
    #[serde(rename = "source_type")]
    pub source_type: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Final attribute or tag name to remap the sources to.
    #[serde(rename = "target")]
    pub target: String,
    /// If the `target_type` of the remapper is `attribute`, try to cast the value to a new specific type.
    /// If the cast is not possible, the original type is kept. `string`, `integer`, or `double` are the possible types.
    /// If the `target_type` is `tag`, this parameter may not be specified.
    #[serde(rename = "target_format")]
    pub target_format: Option<crate::datadogV1::model::TargetFormatType>,
    /// Defines if the final attribute or tag name is from log `attribute` or `tag`.
    #[serde(rename = "target_type")]
    pub target_type: Option<String>,
    /// Type of logs attribute remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsAttributeRemapperType,
}

impl LogsAttributeRemapper {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsAttributeRemapperType,
    ) -> LogsAttributeRemapper {
        LogsAttributeRemapper {
            is_enabled: None,
            name: None,
            override_on_conflict: None,
            preserve_source: None,
            source_type: None,
            sources,
            target,
            target_format: None,
            target_type: None,
            type_,
        }
    }
}

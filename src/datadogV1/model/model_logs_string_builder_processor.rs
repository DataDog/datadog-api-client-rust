// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use the string builder processor to add a new attribute (without spaces or special characters)
/// to a log with the result of the provided template.
/// This enables aggregation of different attributes or raw strings into a single attribute.
///
/// The template is defined by both raw text and blocks with the syntax `%{attribute_path}`.
///
/// **Notes**:
///
/// - The processor only accepts attributes with values or an array of values in the blocks.
/// - If an attribute cannot be used (object or array of object),
///   it is replaced by an empty string or the entire operation is skipped depending on your selection.
/// - If the target attribute already exists, it is overwritten by the result of the template.
/// - Results of the template cannot exceed 256 characters.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsStringBuilderProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// If true, it replaces all missing attributes of `template` by an empty string.
    /// If `false` (default), skips the operation for missing attributes.
    #[serde(rename = "is_replace_missing")]
    pub is_replace_missing: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The name of the attribute that contains the result of the template.
    #[serde(rename = "target")]
    pub target: String,
    /// A formula with one or more attributes and raw text.
    #[serde(rename = "template")]
    pub template: String,
    /// Type of logs string builder processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsStringBuilderProcessorType,
}

impl LogsStringBuilderProcessor {
    pub fn new(
        target: String,
        template: String,
        type_: crate::datadogV1::model::LogsStringBuilderProcessorType,
    ) -> LogsStringBuilderProcessor {
        LogsStringBuilderProcessor {
            is_enabled: None,
            is_replace_missing: None,
            name: None,
            target,
            template,
            type_,
        }
    }

    pub fn with_is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn with_is_replace_missing(&mut self, value: bool) -> &mut Self {
        self.is_replace_missing = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

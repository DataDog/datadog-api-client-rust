// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

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
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn is_replace_missing(mut self, value: bool) -> Self {
        self.is_replace_missing = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsStringBuilderProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsStringBuilderProcessorVisitor;
        impl<'a> Visitor<'a> for LogsStringBuilderProcessorVisitor {
            type Value = LogsStringBuilderProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut is_replace_missing: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut target: Option<String> = None;
                let mut template: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsStringBuilderProcessorType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_replace_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            is_replace_missing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsStringBuilderProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let template = template.ok_or_else(|| M::Error::missing_field("template"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsStringBuilderProcessor {
                    is_enabled,
                    is_replace_missing,
                    name,
                    target,
                    template,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsStringBuilderProcessorVisitor)
    }
}

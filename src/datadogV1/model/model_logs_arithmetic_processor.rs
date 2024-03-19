// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use the Arithmetic Processor to add a new attribute (without spaces or special characters
/// in the new attribute name) to a log with the result of the provided formula.
/// This enables you to remap different time attributes with different units into a single attribute,
/// or to compute operations on attributes within the same log.
///
/// The formula can use parentheses and the basic arithmetic operators `-`, `+`, `*`, `/`.
///
/// By default, the calculation is skipped if an attribute is missing.
/// Select “Replace missing attribute by 0” to automatically populate
/// missing attribute values with 0 to ensure that the calculation is done.
/// An attribute is missing if it is not found in the log attributes,
/// or if it cannot be converted to a number.
///
/// *Notes*:
///
/// - The operator `-` needs to be space split in the formula as it can also be contained in attribute names.
/// - If the target attribute already exists, it is overwritten by the result of the formula.
/// - Results are rounded up to the 9th decimal. For example, if the result of the formula is `0.1234567891`,
///   the actual value stored for the attribute is `0.123456789`.
/// - If you need to scale a unit of measure,
///   see [Scale Filter](<https://docs.datadoghq.com/logs/log_configuration/parsing/?tab=filter#matcher-and-filter>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArithmeticProcessor {
    /// Arithmetic operation between one or more log attributes.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// If `true`, it replaces all missing attributes of expression by `0`, `false`
    /// skip the operation if an attribute is missing.
    #[serde(rename = "is_replace_missing")]
    pub is_replace_missing: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Name of the attribute that contains the result of the arithmetic operation.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs arithmetic processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArithmeticProcessorType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArithmeticProcessor {
    pub fn new(
        expression: String,
        target: String,
        type_: crate::datadogV1::model::LogsArithmeticProcessorType,
    ) -> LogsArithmeticProcessor {
        LogsArithmeticProcessor {
            expression,
            is_enabled: None,
            is_replace_missing: None,
            name: None,
            target,
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

impl<'de> Deserialize<'de> for LogsArithmeticProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArithmeticProcessorVisitor;
        impl<'a> Visitor<'a> for LogsArithmeticProcessorVisitor {
            type Value = LogsArithmeticProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut is_replace_missing: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsArithmeticProcessorType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArithmeticProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArithmeticProcessor {
                    expression,
                    is_enabled,
                    is_replace_missing,
                    name,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArithmeticProcessorVisitor)
    }
}

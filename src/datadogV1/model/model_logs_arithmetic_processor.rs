// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
///   see [Scale Filter](https://docs.datadoghq.com/logs/log_configuration/parsing/?tab=filter#matcher-and-filter).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }
}

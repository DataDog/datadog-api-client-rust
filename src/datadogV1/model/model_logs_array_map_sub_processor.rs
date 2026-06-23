// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A sub-processor used inside an array-map processor.
/// Allowed types: `attribute-remapper`, `string-builder-processor`,
/// `arithmetic-processor`, `category-processor`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsArrayMapSubProcessor {
    LogsArrayMapAttributeRemapper(Box<crate::datadogV1::model::LogsArrayMapAttributeRemapper>),
    LogsArrayMapArithmeticSubProcessor(
        Box<crate::datadogV1::model::LogsArrayMapArithmeticSubProcessor>,
    ),
    LogsArrayMapStringBuilderSubProcessor(
        Box<crate::datadogV1::model::LogsArrayMapStringBuilderSubProcessor>,
    ),
    LogsArrayMapCategorySubProcessor(
        Box<crate::datadogV1::model::LogsArrayMapCategorySubProcessor>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsArrayMapSubProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayMapAttributeRemapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayMapSubProcessor::LogsArrayMapAttributeRemapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayMapArithmeticSubProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayMapSubProcessor::LogsArrayMapArithmeticSubProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayMapStringBuilderSubProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayMapSubProcessor::LogsArrayMapStringBuilderSubProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayMapCategorySubProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayMapSubProcessor::LogsArrayMapCategorySubProcessor(
                    _v,
                ));
            }
        }

        return Ok(LogsArrayMapSubProcessor::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

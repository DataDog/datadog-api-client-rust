// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Configuration of the array processor operation to perform.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsArrayProcessorOperation {
    LogsArrayProcessorOperationAppend(
        Box<crate::datadogV1::model::LogsArrayProcessorOperationAppend>,
    ),
    LogsArrayProcessorOperationLength(
        Box<crate::datadogV1::model::LogsArrayProcessorOperationLength>,
    ),
    LogsArrayProcessorOperationSelect(
        Box<crate::datadogV1::model::LogsArrayProcessorOperationSelect>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsArrayProcessorOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayProcessorOperationAppend>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayProcessorOperation::LogsArrayProcessorOperationAppend(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayProcessorOperationLength>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayProcessorOperation::LogsArrayProcessorOperationLength(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArrayProcessorOperationSelect>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArrayProcessorOperation::LogsArrayProcessorOperationSelect(_v));
            }
        }

        return Ok(LogsArrayProcessorOperation::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

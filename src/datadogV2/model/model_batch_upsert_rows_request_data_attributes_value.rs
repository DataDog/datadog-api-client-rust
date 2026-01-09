// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Types allowed for Reference Table row values.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BatchUpsertRowsRequestDataAttributesValue {
    String(String),
    I32(i32),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for BatchUpsertRowsRequestDataAttributesValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(BatchUpsertRowsRequestDataAttributesValue::String(_v));
        }
        if let Ok(_v) = serde_json::from_value::<i32>(value.clone()) {
            return Ok(BatchUpsertRowsRequestDataAttributesValue::I32(_v));
        }

        return Ok(BatchUpsertRowsRequestDataAttributesValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

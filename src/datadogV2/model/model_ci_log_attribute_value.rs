// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A flat additional log attribute. Objects and arrays are not accepted.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CILogAttributeValue {
    String(String),
    F64(f64),
    Bool(bool),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CILogAttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(CILogAttributeValue::String(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(CILogAttributeValue::F64(_v));
        }
        if let Ok(_v) = serde_json::from_value::<bool>(value.clone()) {
            return Ok(CILogAttributeValue::Bool(_v));
        }

        return Ok(CILogAttributeValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

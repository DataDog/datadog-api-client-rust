// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Value used by the operator in assertions. Can be either a number or string.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsAssertionTargetValue {
    SyntheticsAssertionTargetValueNumber(f64),
    SyntheticsAssertionTargetValueString(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsAssertionTargetValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueString(_v));
        }

        return Ok(SyntheticsAssertionTargetValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

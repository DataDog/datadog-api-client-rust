// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Values used in the step for in multiple step types.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsMobileStepParamsValue {
    SyntheticsMobileStepParamsValueString(String),
    SyntheticsMobileStepParamsValueNumber(i64),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(SyntheticsMobileStepParamsValue::SyntheticsMobileStepParamsValueString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<i64>(value.clone()) {
            return Ok(SyntheticsMobileStepParamsValue::SyntheticsMobileStepParamsValueNumber(_v));
        }

        return Ok(SyntheticsMobileStepParamsValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

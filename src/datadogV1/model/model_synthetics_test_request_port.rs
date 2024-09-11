// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Port to use when performing the test.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsTestRequestPort {
    SyntheticsTestRequestNumericalPort(i64),
    SyntheticsTestRequestVariablePort(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<i64>(value.clone()) {
            return Ok(SyntheticsTestRequestPort::SyntheticsTestRequestNumericalPort(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(SyntheticsTestRequestPort::SyntheticsTestRequestVariablePort(_v));
        }

        return Ok(SyntheticsTestRequestPort::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

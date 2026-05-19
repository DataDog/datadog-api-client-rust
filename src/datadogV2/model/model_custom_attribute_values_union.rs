// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Union of supported value for a custom attribute
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CustomAttributeValuesUnion {
    CustomAttributeStringValue(String),
    CustomAttributeMultiStringValue(Vec<String>),
    CustomAttributeNumberValue(f64),
    CustomAttributeMultiNumberValue(Vec<f64>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CustomAttributeValuesUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(CustomAttributeValuesUnion::CustomAttributeStringValue(_v));
        }
        if let Ok(_v) = serde_json::from_value::<Vec<String>>(value.clone()) {
            return Ok(CustomAttributeValuesUnion::CustomAttributeMultiStringValue(
                _v,
            ));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(CustomAttributeValuesUnion::CustomAttributeNumberValue(_v));
        }
        if let Ok(_v) = serde_json::from_value::<Vec<f64>>(value.clone()) {
            return Ok(CustomAttributeValuesUnion::CustomAttributeMultiNumberValue(
                _v,
            ));
        }

        return Ok(CustomAttributeValuesUnion::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

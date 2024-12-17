// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ComponentGridPropertiesIsVisible` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComponentGridPropertiesIsVisible {
    String(String),
    Bool(bool),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ComponentGridPropertiesIsVisible {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ComponentGridPropertiesIsVisible::String(_v));
        }
        if let Ok(_v) = serde_json::from_value::<bool>(value.clone()) {
            return Ok(ComponentGridPropertiesIsVisible::Bool(_v));
        }

        return Ok(ComponentGridPropertiesIsVisible::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

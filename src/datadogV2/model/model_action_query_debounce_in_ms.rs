// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The minimum time in milliseconds that must pass before the query can be triggered again. This is useful for preventing accidental double-clicks from triggering the query multiple times.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionQueryDebounceInMs {
    F64(f64),
    String(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionQueryDebounceInMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(ActionQueryDebounceInMs::F64(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ActionQueryDebounceInMs::String(_v));
        }

        return Ok(ActionQueryDebounceInMs::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

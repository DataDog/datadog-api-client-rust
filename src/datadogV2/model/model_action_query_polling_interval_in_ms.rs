// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// If specified, the app will poll the query at the specified interval in milliseconds. The minimum polling interval is 15 seconds. The query will only poll when the app's browser tab is active.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionQueryPollingIntervalInMs {
    F64(f64),
    String(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionQueryPollingIntervalInMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(ActionQueryPollingIntervalInMs::F64(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ActionQueryPollingIntervalInMs::String(_v));
        }

        return Ok(ActionQueryPollingIntervalInMs::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

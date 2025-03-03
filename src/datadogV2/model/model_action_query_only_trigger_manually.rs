// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Determines when this query is executed. If set to `false`, the query will run when the app loads and whenever any query arguments change. If set to `true`, the query will only run when manually triggered from elsewhere in the app.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionQueryOnlyTriggerManually {
    Bool(bool),
    String(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionQueryOnlyTriggerManually {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<bool>(value.clone()) {
            return Ok(ActionQueryOnlyTriggerManually::Bool(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ActionQueryOnlyTriggerManually::String(_v));
        }

        return Ok(ActionQueryOnlyTriggerManually::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

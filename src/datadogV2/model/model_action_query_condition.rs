// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Whether to run this query. If specified, the query will only run if this condition evaluates to `true` in JavaScript and all other conditions are also met.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionQueryCondition {
    Bool(bool),
    String(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionQueryCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<bool>(value.clone()) {
            return Ok(ActionQueryCondition::Bool(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ActionQueryCondition::String(_v));
        }

        return Ok(ActionQueryCondition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

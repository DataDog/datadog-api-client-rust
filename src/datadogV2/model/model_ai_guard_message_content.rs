// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The message content, either a plain string or an array of content parts.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AIGuardMessageContent {
    String(String),
    AIGuardContentPartList(Vec<crate::datadogV2::model::AIGuardContentPart>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AIGuardMessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(AIGuardMessageContent::String(_v));
        }
        if let Ok(_v) = serde_json::from_value::<Vec<crate::datadogV2::model::AIGuardContentPart>>(
            value.clone(),
        ) {
            return Ok(AIGuardMessageContent::AIGuardContentPartList(_v));
        }

        return Ok(AIGuardMessageContent::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

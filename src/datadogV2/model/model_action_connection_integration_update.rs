// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ActionConnectionIntegrationUpdate` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionConnectionIntegrationUpdate {
    AWSIntegrationUpdate(Box<crate::datadogV2::model::AWSIntegrationUpdate>),
    HTTPIntegrationUpdate(Box<crate::datadogV2::model::HTTPIntegrationUpdate>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionConnectionIntegrationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AWSIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::AWSIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::HTTPIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::HTTPIntegrationUpdate(_v));
            }
        }

        return Ok(ActionConnectionIntegrationUpdate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

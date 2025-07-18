// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ActionConnectionIntegration` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionConnectionIntegration {
    AWSIntegration(Box<crate::datadogV2::model::AWSIntegration>),
    DatadogIntegration(Box<crate::datadogV2::model::DatadogIntegration>),
    HTTPIntegration(Box<crate::datadogV2::model::HTTPIntegration>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionConnectionIntegration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AWSIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::AWSIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::DatadogIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::DatadogIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::HTTPIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::HTTPIntegration(_v));
            }
        }

        return Ok(ActionConnectionIntegration::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

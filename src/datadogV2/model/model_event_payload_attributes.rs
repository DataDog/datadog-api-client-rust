// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// JSON object for custom attributes. Schema is different per event category.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EventPayloadAttributes {
    ChangeEventCustomAttributes(Box<crate::datadogV2::model::ChangeEventCustomAttributes>),
    AlertEventCustomAttributes(Box<crate::datadogV2::model::AlertEventCustomAttributes>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EventPayloadAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ChangeEventCustomAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(EventPayloadAttributes::ChangeEventCustomAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AlertEventCustomAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(EventPayloadAttributes::AlertEventCustomAttributes(_v));
            }
        }

        return Ok(EventPayloadAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

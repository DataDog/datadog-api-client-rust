// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// JSON object for category-specific attributes.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum V2EventAttributesAttributes {
    ChangeEventAttributes(Box<crate::datadogV2::model::ChangeEventAttributes>),
    AlertEventAttributes(Box<crate::datadogV2::model::AlertEventAttributes>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for V2EventAttributesAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ChangeEventAttributes>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(V2EventAttributesAttributes::ChangeEventAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AlertEventAttributes>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(V2EventAttributesAttributes::AlertEventAttributes(_v));
            }
        }

        return Ok(V2EventAttributesAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

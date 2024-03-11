// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Dynamic fields for which selections can be made, with field names as keys.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncidentFieldAttributes {
    IncidentFieldAttributesSingleValue(
        Box<crate::datadogV2::model::IncidentFieldAttributesSingleValue>,
    ),
    IncidentFieldAttributesMultipleValue(
        Box<crate::datadogV2::model::IncidentFieldAttributesMultipleValue>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for IncidentFieldAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentFieldAttributesSingleValue>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentFieldAttributes::IncidentFieldAttributesSingleValue(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentFieldAttributesMultipleValue>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentFieldAttributes::IncidentFieldAttributesMultipleValue(_v));
            }
        }

        return Ok(IncidentFieldAttributes::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}

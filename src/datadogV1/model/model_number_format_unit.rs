// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Number format unit.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NumberFormatUnit {
    NumberFormatUnitCanonical(Box<crate::datadogV1::model::NumberFormatUnitCanonical>),
    NumberFormatUnitCustom(Box<crate::datadogV1::model::NumberFormatUnitCustom>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for NumberFormatUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NumberFormatUnitCanonical>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NumberFormatUnit::NumberFormatUnitCanonical(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::NumberFormatUnitCustom>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(NumberFormatUnit::NumberFormatUnitCustom(_v));
            }
        }

        return Ok(NumberFormatUnit::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

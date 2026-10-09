// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Severity override to apply to the findings.
/// Set `action` to `set` to apply a manual severity override with the given `value`.
/// Set `action` to `clear` to remove a manual severity override.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SeverityOverrideAttributes {
    SeverityOverrideSet(Box<crate::datadogV2::model::SeverityOverrideSet>),
    SeverityOverrideClear(Box<crate::datadogV2::model::SeverityOverrideClear>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SeverityOverrideAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SeverityOverrideSet>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SeverityOverrideAttributes::SeverityOverrideSet(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SeverityOverrideClear>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SeverityOverrideAttributes::SeverityOverrideClear(_v));
            }
        }

        return Ok(SeverityOverrideAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

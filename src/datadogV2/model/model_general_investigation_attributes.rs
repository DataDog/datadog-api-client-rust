// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Attributes for a general investigation, not tied to a specific monitor alert.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GeneralInvestigationAttributes {
    GeneralInvestigationAttributesWithoutTimeBounds(
        Box<crate::datadogV2::model::GeneralInvestigationAttributesWithoutTimeBounds>,
    ),
    GeneralInvestigationAttributesWithTimeBounds(
        Box<crate::datadogV2::model::GeneralInvestigationAttributesWithTimeBounds>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GeneralInvestigationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GeneralInvestigationAttributesWithoutTimeBounds>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    GeneralInvestigationAttributes::GeneralInvestigationAttributesWithoutTimeBounds(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GeneralInvestigationAttributesWithTimeBounds>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    GeneralInvestigationAttributes::GeneralInvestigationAttributesWithTimeBounds(
                        _v,
                    ),
                );
            }
        }

        return Ok(GeneralInvestigationAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

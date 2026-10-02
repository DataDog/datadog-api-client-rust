// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsExperimentV2DTODataAttributesConclusionOutcome {
    POSITIVE,
    NEGATIVE,
    NEUTRAL,
    INCONCLUSIVE,
    MISCONFIGURED,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsExperimentV2DTODataAttributesConclusionOutcome {
    fn to_string(&self) -> String {
        match self {
            Self::POSITIVE => String::from("POSITIVE"),
            Self::NEGATIVE => String::from("NEGATIVE"),
            Self::NEUTRAL => String::from("NEUTRAL"),
            Self::INCONCLUSIVE => String::from("INCONCLUSIVE"),
            Self::MISCONFIGURED => String::from("MISCONFIGURED"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsExperimentV2DTODataAttributesConclusionOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ExperimentsExperimentV2DTODataAttributesConclusionOutcome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "POSITIVE" => Self::POSITIVE,
            "NEGATIVE" => Self::NEGATIVE,
            "NEUTRAL" => Self::NEUTRAL,
            "INCONCLUSIVE" => Self::INCONCLUSIVE,
            "MISCONFIGURED" => Self::MISCONFIGURED,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

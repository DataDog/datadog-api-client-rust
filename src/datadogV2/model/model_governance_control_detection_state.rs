// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceControlDetectionState {
    ACTIVE,
    EXCEPTION,
    MITIGATED,
    INACTIVE,
    OBSOLETE,
    RESOLVED_EXTERNALLY,
    MITIGATION_IN_PROGRESS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for GovernanceControlDetectionState {
    fn to_string(&self) -> String {
        match self {
            Self::ACTIVE => String::from("active"),
            Self::EXCEPTION => String::from("exception"),
            Self::MITIGATED => String::from("mitigated"),
            Self::INACTIVE => String::from("inactive"),
            Self::OBSOLETE => String::from("obsolete"),
            Self::RESOLVED_EXTERNALLY => String::from("resolved_externally"),
            Self::MITIGATION_IN_PROGRESS => String::from("mitigation_in_progress"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for GovernanceControlDetectionState {
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

impl<'de> Deserialize<'de> for GovernanceControlDetectionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "active" => Self::ACTIVE,
            "exception" => Self::EXCEPTION,
            "mitigated" => Self::MITIGATED,
            "inactive" => Self::INACTIVE,
            "obsolete" => Self::OBSOLETE,
            "resolved_externally" => Self::RESOLVED_EXTERNALLY,
            "mitigation_in_progress" => Self::MITIGATION_IN_PROGRESS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

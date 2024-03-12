// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentSeverity {
    UNKNOWN,
    SEV_1,
    SEV_2,
    SEV_3,
    SEV_4,
    SEV_5,
}

impl ToString for IncidentSeverity {
    fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::SEV_1 => String::from("SEV-1"),
            Self::SEV_2 => String::from("SEV-2"),
            Self::SEV_3 => String::from("SEV-3"),
            Self::SEV_4 => String::from("SEV-4"),
            Self::SEV_5 => String::from("SEV-5"),
        }
    }
}

impl Serialize for IncidentSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for IncidentSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "UNKNOWN" => Self::UNKNOWN,
            "SEV-1" => Self::SEV_1,
            "SEV-2" => Self::SEV_2,
            "SEV-3" => Self::SEV_3,
            "SEV-4" => Self::SEV_4,
            "SEV-5" => Self::SEV_5,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

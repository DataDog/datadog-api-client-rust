// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SLOState {
    BREACHED,
    WARNING,
    OK,
    NO_DATA,
}

impl ToString for SLOState {
    fn to_string(&self) -> String {
        match self {
            Self::BREACHED => String::from("breached"),
            Self::WARNING => String::from("warning"),
            Self::OK => String::from("ok"),
            Self::NO_DATA => String::from("no_data"),
        }
    }
}

impl Serialize for SLOState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SLOState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "breached" => Self::BREACHED,
            "warning" => Self::WARNING,
            "ok" => Self::OK,
            "no_data" => Self::NO_DATA,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

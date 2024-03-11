// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GetTeamMembershipsSort {
    MANAGER_NAME,
    _MANAGER_NAME,
    NAME,
    _NAME,
    HANDLE,
    _HANDLE,
    EMAIL,
    _EMAIL,
}

impl ToString for GetTeamMembershipsSort {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGER_NAME => String::from("manager_name"),
            Self::_MANAGER_NAME => String::from("-manager_name"),
            Self::NAME => String::from("name"),
            Self::_NAME => String::from("-name"),
            Self::HANDLE => String::from("handle"),
            Self::_HANDLE => String::from("-handle"),
            Self::EMAIL => String::from("email"),
            Self::_EMAIL => String::from("-email"),
        }
    }
}

impl Serialize for GetTeamMembershipsSort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for GetTeamMembershipsSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "manager_name" => Self::MANAGER_NAME,
            "-manager_name" => Self::_MANAGER_NAME,
            "name" => Self::NAME,
            "-name" => Self::_NAME,
            "handle" => Self::HANDLE,
            "-handle" => Self::_HANDLE,
            "email" => Self::EMAIL,
            "-email" => Self::_EMAIL,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

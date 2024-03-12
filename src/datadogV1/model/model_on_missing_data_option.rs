// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnMissingDataOption {
    DEFAULT,
    SHOW_NO_DATA,
    SHOW_AND_NOTIFY_NO_DATA,
    RESOLVE,
}

impl ToString for OnMissingDataOption {
    fn to_string(&self) -> String {
        match self {
            Self::DEFAULT => String::from("default"),
            Self::SHOW_NO_DATA => String::from("show_no_data"),
            Self::SHOW_AND_NOTIFY_NO_DATA => String::from("show_and_notify_no_data"),
            Self::RESOLVE => String::from("resolve"),
        }
    }
}

impl Serialize for OnMissingDataOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for OnMissingDataOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => Self::DEFAULT,
            "show_no_data" => Self::SHOW_NO_DATA,
            "show_and_notify_no_data" => Self::SHOW_AND_NOTIFY_NO_DATA,
            "resolve" => Self::RESOLVE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

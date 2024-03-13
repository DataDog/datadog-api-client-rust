// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetOrderBy {
    CHANGE,
    NAME,
    PRESENT,
    PAST,
}

impl ToString for WidgetOrderBy {
    fn to_string(&self) -> String {
        match self {
            Self::CHANGE => String::from("change"),
            Self::NAME => String::from("name"),
            Self::PRESENT => String::from("present"),
            Self::PAST => String::from("past"),
        }
    }
}

impl Serialize for WidgetOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetOrderBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "change" => Self::CHANGE,
            "name" => Self::NAME,
            "present" => Self::PRESENT,
            "past" => Self::PAST,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

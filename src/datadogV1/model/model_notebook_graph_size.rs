// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NotebookGraphSize {
    EXTRA_SMALL,
    SMALL,
    MEDIUM,
    LARGE,
    EXTRA_LARGE,
}

impl ToString for NotebookGraphSize {
    fn to_string(&self) -> String {
        match self {
            Self::EXTRA_SMALL => String::from("xs"),
            Self::SMALL => String::from("s"),
            Self::MEDIUM => String::from("m"),
            Self::LARGE => String::from("l"),
            Self::EXTRA_LARGE => String::from("xl"),
        }
    }
}

impl Serialize for NotebookGraphSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for NotebookGraphSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "xs" => Self::EXTRA_SMALL,
            "s" => Self::SMALL,
            "m" => Self::MEDIUM,
            "l" => Self::LARGE,
            "xl" => Self::EXTRA_LARGE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

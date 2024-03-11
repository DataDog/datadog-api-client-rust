// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsageSort {
    COMPUTED_ON,
    SIZE,
    START_DATE,
    END_DATE,
}

impl ToString for UsageSort {
    fn to_string(&self) -> String {
        match self {
            Self::COMPUTED_ON => String::from("computed_on"),
            Self::SIZE => String::from("size"),
            Self::START_DATE => String::from("start_date"),
            Self::END_DATE => String::from("end_date"),
        }
    }
}

impl Serialize for UsageSort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for UsageSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "computed_on" => Self::COMPUTED_ON,
            "size" => Self::SIZE,
            "start_date" => Self::START_DATE,
            "end_date" => Self::END_DATE,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}

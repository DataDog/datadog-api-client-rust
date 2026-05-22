// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SampleLogGenerationDuration {
    ONE_HOUR,
    ONE_DAY,
    THREE_DAYS,
    SEVEN_DAYS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SampleLogGenerationDuration {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_HOUR => String::from("1h"),
            Self::ONE_DAY => String::from("1d"),
            Self::THREE_DAYS => String::from("3d"),
            Self::SEVEN_DAYS => String::from("7d"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SampleLogGenerationDuration {
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

impl<'de> Deserialize<'de> for SampleLogGenerationDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "1h" => Self::ONE_HOUR,
            "1d" => Self::ONE_DAY,
            "3d" => Self::THREE_DAYS,
            "7d" => Self::SEVEN_DAYS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

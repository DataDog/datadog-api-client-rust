// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SLOTimeframe {
    SEVEN_DAYS,
    THIRTY_DAYS,
    NINETY_DAYS,
    CUSTOM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SLOTimeframe {
    fn to_string(&self) -> String {
        match self {
            Self::SEVEN_DAYS => String::from("7d"),
            Self::THIRTY_DAYS => String::from("30d"),
            Self::NINETY_DAYS => String::from("90d"),
            Self::CUSTOM => String::from("custom"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SLOTimeframe {
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

impl<'de> Deserialize<'de> for SLOTimeframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "7d" => Self::SEVEN_DAYS,
            "30d" => Self::THIRTY_DAYS,
            "90d" => Self::NINETY_DAYS,
            "custom" => Self::CUSTOM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

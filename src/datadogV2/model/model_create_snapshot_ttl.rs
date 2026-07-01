// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreateSnapshotTTL {
    THIRTY_DAYS,
    SIXTY_DAYS,
    NINETY_DAYS,
    ONE_YEAR,
    TWO_YEARS,
    INFINITE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CreateSnapshotTTL {
    fn to_string(&self) -> String {
        match self {
            Self::THIRTY_DAYS => String::from("30d"),
            Self::SIXTY_DAYS => String::from("60d"),
            Self::NINETY_DAYS => String::from("90d"),
            Self::ONE_YEAR => String::from("1y"),
            Self::TWO_YEARS => String::from("2y"),
            Self::INFINITE => String::from("inf"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CreateSnapshotTTL {
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

impl<'de> Deserialize<'de> for CreateSnapshotTTL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "30d" => Self::THIRTY_DAYS,
            "60d" => Self::SIXTY_DAYS,
            "90d" => Self::NINETY_DAYS,
            "1y" => Self::ONE_YEAR,
            "2y" => Self::TWO_YEARS,
            "inf" => Self::INFINITE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

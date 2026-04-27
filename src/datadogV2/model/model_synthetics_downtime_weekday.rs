// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsDowntimeWeekday {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsDowntimeWeekday {
    fn to_string(&self) -> String {
        match self {
            Self::MONDAY => String::from("MO"),
            Self::TUESDAY => String::from("TU"),
            Self::WEDNESDAY => String::from("WE"),
            Self::THURSDAY => String::from("TH"),
            Self::FRIDAY => String::from("FR"),
            Self::SATURDAY => String::from("SA"),
            Self::SUNDAY => String::from("SU"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsDowntimeWeekday {
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

impl<'de> Deserialize<'de> for SyntheticsDowntimeWeekday {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "MO" => Self::MONDAY,
            "TU" => Self::TUESDAY,
            "WE" => Self::WEDNESDAY,
            "TH" => Self::THURSDAY,
            "FR" => Self::FRIDAY,
            "SA" => Self::SATURDAY,
            "SU" => Self::SUNDAY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

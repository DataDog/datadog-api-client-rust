// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringContentPackTimestampBucket {
    NOT_SEEN,
    WITHIN_24_HOURS,
    WITHIN_24_TO_72_HOURS,
    OVER_72H_TO_30D,
    OVER_30D,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringContentPackTimestampBucket {
    fn to_string(&self) -> String {
        match self {
            Self::NOT_SEEN => String::from("not_seen"),
            Self::WITHIN_24_HOURS => String::from("within_24_hours"),
            Self::WITHIN_24_TO_72_HOURS => String::from("within_24_to_72_hours"),
            Self::OVER_72H_TO_30D => String::from("over_72h_to_30d"),
            Self::OVER_30D => String::from("over_30d"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringContentPackTimestampBucket {
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackTimestampBucket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "not_seen" => Self::NOT_SEEN,
            "within_24_hours" => Self::WITHIN_24_HOURS,
            "within_24_to_72_hours" => Self::WITHIN_24_TO_72_HOURS,
            "over_72h_to_30d" => Self::OVER_72H_TO_30D,
            "over_30d" => Self::OVER_30D,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

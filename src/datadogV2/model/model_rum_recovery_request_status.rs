// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RumRecoveryRequestStatus {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED,
    CANCELLED,
    EXPIRED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RumRecoveryRequestStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING => String::from("PENDING"),
            Self::PROCESSING => String::from("PROCESSING"),
            Self::COMPLETED => String::from("COMPLETED"),
            Self::FAILED => String::from("FAILED"),
            Self::CANCELLED => String::from("CANCELLED"),
            Self::EXPIRED => String::from("EXPIRED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RumRecoveryRequestStatus {
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

impl<'de> Deserialize<'de> for RumRecoveryRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PENDING" => Self::PENDING,
            "PROCESSING" => Self::PROCESSING,
            "COMPLETED" => Self::COMPLETED,
            "FAILED" => Self::FAILED,
            "CANCELLED" => Self::CANCELLED,
            "EXPIRED" => Self::EXPIRED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

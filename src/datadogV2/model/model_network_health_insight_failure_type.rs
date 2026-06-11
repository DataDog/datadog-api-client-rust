// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkHealthInsightFailureType {
    TIMEOUT,
    NXDOMAIN,
    SERVFAIL,
    GENERAL_FAILURE,
    EXPIRED,
    EXPIRING_SOON,
    DENIED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for NetworkHealthInsightFailureType {
    fn to_string(&self) -> String {
        match self {
            Self::TIMEOUT => String::from("timeout"),
            Self::NXDOMAIN => String::from("nxdomain"),
            Self::SERVFAIL => String::from("servfail"),
            Self::GENERAL_FAILURE => String::from("general_failure"),
            Self::EXPIRED => String::from("expired"),
            Self::EXPIRING_SOON => String::from("expiring_soon"),
            Self::DENIED => String::from("denied"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for NetworkHealthInsightFailureType {
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

impl<'de> Deserialize<'de> for NetworkHealthInsightFailureType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "timeout" => Self::TIMEOUT,
            "nxdomain" => Self::NXDOMAIN,
            "servfail" => Self::SERVFAIL,
            "general_failure" => Self::GENERAL_FAILURE,
            "expired" => Self::EXPIRED,
            "expiring_soon" => Self::EXPIRING_SOON,
            "denied" => Self::DENIED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestDetailsSubType {
    HTTP,
    SSL,
    TCP,
    DNS,
    MULTI,
    ICMP,
    UDP,
    WEBSOCKET,
    GRPC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsTestDetailsSubType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP => String::from("http"),
            Self::SSL => String::from("ssl"),
            Self::TCP => String::from("tcp"),
            Self::DNS => String::from("dns"),
            Self::MULTI => String::from("multi"),
            Self::ICMP => String::from("icmp"),
            Self::UDP => String::from("udp"),
            Self::WEBSOCKET => String::from("websocket"),
            Self::GRPC => String::from("grpc"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsTestDetailsSubType {
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

impl<'de> Deserialize<'de> for SyntheticsTestDetailsSubType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "http" => Self::HTTP,
            "ssl" => Self::SSL,
            "tcp" => Self::TCP,
            "dns" => Self::DNS,
            "multi" => Self::MULTI,
            "icmp" => Self::ICMP,
            "udp" => Self::UDP,
            "websocket" => Self::WEBSOCKET,
            "grpc" => Self::GRPC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

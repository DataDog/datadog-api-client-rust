// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HamrOrgConnectionStatus {
    UNSPECIFIED,
    ONBOARDING,
    PASSIVE,
    FAILOVER,
    ACTIVE,
    RECOVERY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for HamrOrgConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::UNSPECIFIED => serializer.serialize_i32(0),
            Self::ONBOARDING => serializer.serialize_i32(1),
            Self::PASSIVE => serializer.serialize_i32(2),
            Self::FAILOVER => serializer.serialize_i32(3),
            Self::ACTIVE => serializer.serialize_i32(4),
            Self::RECOVERY => serializer.serialize_i32(5),
        }
    }
}

impl<'de> Deserialize<'de> for HamrOrgConnectionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            0 => Self::UNSPECIFIED,
            1 => Self::ONBOARDING,
            2 => Self::PASSIVE,
            3 => Self::FAILOVER,
            4 => Self::ACTIVE,
            5 => Self::RECOVERY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}

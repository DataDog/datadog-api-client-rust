// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPageReleaseOrigin {
    ACK_TIMEOUT,
    TEAM_RULES_ACTIVE_HOURS,
    MANUAL_SNOOZE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPageReleaseOrigin {
    fn to_string(&self) -> String {
        match self {
            Self::ACK_TIMEOUT => String::from("ack_timeout"),
            Self::TEAM_RULES_ACTIVE_HOURS => String::from("team_rules_active_hours"),
            Self::MANUAL_SNOOZE => String::from("manual_snooze"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPageReleaseOrigin {
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

impl<'de> Deserialize<'de> for OnCallPageReleaseOrigin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ack_timeout" => Self::ACK_TIMEOUT,
            "team_rules_active_hours" => Self::TEAM_RULES_ACTIVE_HOURS,
            "manual_snooze" => Self::MANUAL_SNOOZE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

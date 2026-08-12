// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NotificationRuleTargetType {
    EMAIL,
    SLACK_CHANNEL,
    SLACK_USER,
    WEBHOOK,
    PAGERDUTY_SERVICE,
    MS_TEAMS_CHANNEL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for NotificationRuleTargetType {
    fn to_string(&self) -> String {
        match self {
            Self::EMAIL => String::from("EMAIL"),
            Self::SLACK_CHANNEL => String::from("SLACK_CHANNEL"),
            Self::SLACK_USER => String::from("SLACK_USER"),
            Self::WEBHOOK => String::from("WEBHOOK"),
            Self::PAGERDUTY_SERVICE => String::from("PAGERDUTY_SERVICE"),
            Self::MS_TEAMS_CHANNEL => String::from("MS_TEAMS_CHANNEL"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for NotificationRuleTargetType {
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

impl<'de> Deserialize<'de> for NotificationRuleTargetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "EMAIL" => Self::EMAIL,
            "SLACK_CHANNEL" => Self::SLACK_CHANNEL,
            "SLACK_USER" => Self::SLACK_USER,
            "WEBHOOK" => Self::WEBHOOK,
            "PAGERDUTY_SERVICE" => Self::PAGERDUTY_SERVICE,
            "MS_TEAMS_CHANNEL" => Self::MS_TEAMS_CHANNEL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

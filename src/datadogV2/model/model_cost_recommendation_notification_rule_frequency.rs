// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CostRecommendationNotificationRuleFrequency {
    WEEKLY,
    BIWEEKLY,
    EVERY_30,
    EVERY_90,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CostRecommendationNotificationRuleFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::WEEKLY => String::from("weekly"),
            Self::BIWEEKLY => String::from("biweekly"),
            Self::EVERY_30 => String::from("every_30"),
            Self::EVERY_90 => String::from("every_90"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CostRecommendationNotificationRuleFrequency {
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

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleFrequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "weekly" => Self::WEEKLY,
            "biweekly" => Self::BIWEEKLY,
            "every_30" => Self::EVERY_30,
            "every_90" => Self::EVERY_90,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

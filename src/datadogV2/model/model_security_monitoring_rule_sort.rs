// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringRuleSort {
    NAME,
    CREATION_DATE,
    UPDATE_DATE,
    ENABLED,
    TYPE,
    HIGHEST_SEVERITY,
    SOURCE,
    NAME_DESCENDING,
    CREATION_DATE_DESCENDING,
    UPDATE_DATE_DESCENDING,
    ENABLED_DESCENDING,
    TYPE_DESCENDING,
    HIGHEST_SEVERITY_DESCENDING,
    SOURCE_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringRuleSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::CREATION_DATE => String::from("creation_date"),
            Self::UPDATE_DATE => String::from("update_date"),
            Self::ENABLED => String::from("enabled"),
            Self::TYPE => String::from("type"),
            Self::HIGHEST_SEVERITY => String::from("highest_severity"),
            Self::SOURCE => String::from("source"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::CREATION_DATE_DESCENDING => String::from("-creation_date"),
            Self::UPDATE_DATE_DESCENDING => String::from("-update_date"),
            Self::ENABLED_DESCENDING => String::from("-enabled"),
            Self::TYPE_DESCENDING => String::from("-type"),
            Self::HIGHEST_SEVERITY_DESCENDING => String::from("-highest_severity"),
            Self::SOURCE_DESCENDING => String::from("-source"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringRuleSort {
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "creation_date" => Self::CREATION_DATE,
            "update_date" => Self::UPDATE_DATE,
            "enabled" => Self::ENABLED,
            "type" => Self::TYPE,
            "highest_severity" => Self::HIGHEST_SEVERITY,
            "source" => Self::SOURCE,
            "-name" => Self::NAME_DESCENDING,
            "-creation_date" => Self::CREATION_DATE_DESCENDING,
            "-update_date" => Self::UPDATE_DATE_DESCENDING,
            "-enabled" => Self::ENABLED_DESCENDING,
            "-type" => Self::TYPE_DESCENDING,
            "-highest_severity" => Self::HIGHEST_SEVERITY_DESCENDING,
            "-source" => Self::SOURCE_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

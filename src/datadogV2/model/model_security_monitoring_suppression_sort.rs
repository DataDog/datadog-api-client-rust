// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringSuppressionSort {
    NAME,
    START_DATE,
    EXPIRATION_DATE,
    UPDATE_DATE,
    ENABLED,
    NAME_DESCENDING,
    START_DATE_DESCENDING,
    EXPIRATION_DATE_DESCENDING,
    UPDATE_DATE_DESCENDING,
    CREATION_DATE_DESCENDING,
    ENABLED_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringSuppressionSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::START_DATE => String::from("start_date"),
            Self::EXPIRATION_DATE => String::from("expiration_date"),
            Self::UPDATE_DATE => String::from("update_date"),
            Self::ENABLED => String::from("enabled"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::START_DATE_DESCENDING => String::from("-start_date"),
            Self::EXPIRATION_DATE_DESCENDING => String::from("-expiration_date"),
            Self::UPDATE_DATE_DESCENDING => String::from("-update_date"),
            Self::CREATION_DATE_DESCENDING => String::from("-creation_date"),
            Self::ENABLED_DESCENDING => String::from("-enabled"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringSuppressionSort {
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

impl<'de> Deserialize<'de> for SecurityMonitoringSuppressionSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "start_date" => Self::START_DATE,
            "expiration_date" => Self::EXPIRATION_DATE,
            "update_date" => Self::UPDATE_DATE,
            "enabled" => Self::ENABLED,
            "-name" => Self::NAME_DESCENDING,
            "-start_date" => Self::START_DATE_DESCENDING,
            "-expiration_date" => Self::EXPIRATION_DATE_DESCENDING,
            "-update_date" => Self::UPDATE_DATE_DESCENDING,
            "-creation_date" => Self::CREATION_DATE_DESCENDING,
            "-enabled" => Self::ENABLED_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

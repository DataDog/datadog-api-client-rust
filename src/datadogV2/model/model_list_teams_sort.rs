// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListTeamsSort {
    NAME,
    _NAME,
    USER_COUNT,
    _USER_COUNT,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for ListTeamsSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::_NAME => String::from("-name"),
            Self::USER_COUNT => String::from("user_count"),
            Self::_USER_COUNT => String::from("-user_count"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ListTeamsSort {
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

impl<'de> Deserialize<'de> for ListTeamsSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "-name" => Self::_NAME,
            "user_count" => Self::USER_COUNT,
            "-user_count" => Self::_USER_COUNT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

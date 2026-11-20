// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueStreamColumnName {
	SERVICE,
	COUNT,
	VOLUME,
	RESOURCES,
	ERROR_TYPE,
	ERROR_MESSAGE,
	FILE_PATH,
	LAST_SEEN,
	FIRST_SEEN,
	LANGUAGE,
	PLATFORM,
	DETAILS,
	IMPACTED_SESSIONS,
	IMPACTED_USERS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueStreamColumnName {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICE => String::from("service"),
            Self::COUNT => String::from("count"),
            Self::VOLUME => String::from("volume"),
            Self::RESOURCES => String::from("resources"),
            Self::ERROR_TYPE => String::from("error_type"),
            Self::ERROR_MESSAGE => String::from("error_message"),
            Self::FILE_PATH => String::from("file_path"),
            Self::LAST_SEEN => String::from("last_seen"),
            Self::FIRST_SEEN => String::from("first_seen"),
            Self::LANGUAGE => String::from("language"),
            Self::PLATFORM => String::from("platform"),
            Self::DETAILS => String::from("details"),
            Self::IMPACTED_SESSIONS => String::from("impacted_sessions"),
            Self::IMPACTED_USERS => String::from("impacted_users"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssueStreamColumnName {
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

impl<'de> Deserialize<'de> for IssueStreamColumnName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"service" => Self::SERVICE,"count" => Self::COUNT,"volume" => Self::VOLUME,"resources" => Self::RESOURCES,"error_type" => Self::ERROR_TYPE,"error_message" => Self::ERROR_MESSAGE,"file_path" => Self::FILE_PATH,"last_seen" => Self::LAST_SEEN,"first_seen" => Self::FIRST_SEEN,"language" => Self::LANGUAGE,"platform" => Self::PLATFORM,"details" => Self::DETAILS,"impacted_sessions" => Self::IMPACTED_SESSIONS,"impacted_users" => Self::IMPACTED_USERS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

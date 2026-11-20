// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueStreamState {
	OPEN,
	RESOLVED,
	IGNORED,
	EXCLUDED,
	IN_PROGRESS,
	ACKNOWLEDGED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueStreamState {
    fn to_string(&self) -> String {
        match self {
            Self::OPEN => String::from("OPEN"),
            Self::RESOLVED => String::from("RESOLVED"),
            Self::IGNORED => String::from("IGNORED"),
            Self::EXCLUDED => String::from("EXCLUDED"),
            Self::IN_PROGRESS => String::from("IN_PROGRESS"),
            Self::ACKNOWLEDGED => String::from("ACKNOWLEDGED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssueStreamState {
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

impl<'de> Deserialize<'de> for IssueStreamState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"OPEN" => Self::OPEN,"RESOLVED" => Self::RESOLVED,"IGNORED" => Self::IGNORED,"EXCLUDED" => Self::EXCLUDED,"IN_PROGRESS" => Self::IN_PROGRESS,"ACKNOWLEDGED" => Self::ACKNOWLEDGED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

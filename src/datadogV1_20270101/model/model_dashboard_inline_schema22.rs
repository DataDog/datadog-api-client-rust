// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardInlineSchema22 {
	AUTO,
	COMPACT,
	FULL,
	FIT_CONTENT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DashboardInlineSchema22 {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::COMPACT => String::from("compact"),
            Self::FULL => String::from("full"),
            Self::FIT_CONTENT => String::from("fit-content"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DashboardInlineSchema22 {
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

impl<'de> Deserialize<'de> for DashboardInlineSchema22 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"auto" => Self::AUTO,"compact" => Self::COMPACT,"full" => Self::FULL,"fit-content" => Self::FIT_CONTENT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExcludeNullTableColumnFunctionName {
	EXCLUDE_NULL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExcludeNullTableColumnFunctionName {
    fn to_string(&self) -> String {
        match self {
            Self::EXCLUDE_NULL => String::from("exclude_null"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExcludeNullTableColumnFunctionName {
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

impl<'de> Deserialize<'de> for ExcludeNullTableColumnFunctionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"exclude_null" => Self::EXCLUDE_NULL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

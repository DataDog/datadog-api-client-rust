// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnowflakeQueryDatasetDataSource {
	SNOWFLAKE_QUERY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SnowflakeQueryDatasetDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::SNOWFLAKE_QUERY => String::from("snowflake_query"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SnowflakeQueryDatasetDataSource {
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

impl<'de> Deserialize<'de> for SnowflakeQueryDatasetDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"snowflake_query" => Self::SNOWFLAKE_QUERY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductAnalyticsExecutionType {
    SIMPLE,
    BACKGROUND,
    TRINO_MULTISTEP,
    MATERIALIZED_VIEW,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ProductAnalyticsExecutionType {
    fn to_string(&self) -> String {
        match self {
            Self::SIMPLE => String::from("simple"),
            Self::BACKGROUND => String::from("background"),
            Self::TRINO_MULTISTEP => String::from("trino-multistep"),
            Self::MATERIALIZED_VIEW => String::from("materialized-view"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ProductAnalyticsExecutionType {
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

impl<'de> Deserialize<'de> for ProductAnalyticsExecutionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple" => Self::SIMPLE,
            "background" => Self::BACKGROUND,
            "trino-multistep" => Self::TRINO_MULTISTEP,
            "materialized-view" => Self::MATERIALIZED_VIEW,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

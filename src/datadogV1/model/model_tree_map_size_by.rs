// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeMapSizeBy {
    PCT_CPU,
    PCT_MEM,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for TreeMapSizeBy {
    fn to_string(&self) -> String {
        match self {
            Self::PCT_CPU => String::from("pct_cpu"),
            Self::PCT_MEM => String::from("pct_mem"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TreeMapSizeBy {
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

impl<'de> Deserialize<'de> for TreeMapSizeBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pct_cpu" => Self::PCT_CPU,
            "pct_mem" => Self::PCT_MEM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

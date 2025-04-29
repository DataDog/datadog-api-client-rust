// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineReduceProcessorMergeStrategyStrategy {
    DISCARD,
    RETAIN,
    SUM,
    MAX,
    MIN,
    ARRAY,
    CONCAT,
    CONCAT_NEWLINE,
    CONCAT_RAW,
    SHORTEST_ARRAY,
    LONGEST_ARRAY,
    FLAT_UNIQUE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineReduceProcessorMergeStrategyStrategy {
    fn to_string(&self) -> String {
        match self {
            Self::DISCARD => String::from("discard"),
            Self::RETAIN => String::from("retain"),
            Self::SUM => String::from("sum"),
            Self::MAX => String::from("max"),
            Self::MIN => String::from("min"),
            Self::ARRAY => String::from("array"),
            Self::CONCAT => String::from("concat"),
            Self::CONCAT_NEWLINE => String::from("concat_newline"),
            Self::CONCAT_RAW => String::from("concat_raw"),
            Self::SHORTEST_ARRAY => String::from("shortest_array"),
            Self::LONGEST_ARRAY => String::from("longest_array"),
            Self::FLAT_UNIQUE => String::from("flat_unique"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineReduceProcessorMergeStrategyStrategy {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineReduceProcessorMergeStrategyStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "discard" => Self::DISCARD,
            "retain" => Self::RETAIN,
            "sum" => Self::SUM,
            "max" => Self::MAX,
            "min" => Self::MIN,
            "array" => Self::ARRAY,
            "concat" => Self::CONCAT,
            "concat_newline" => Self::CONCAT_NEWLINE,
            "concat_raw" => Self::CONCAT_RAW,
            "shortest_array" => Self::SHORTEST_ARRAY,
            "longest_array" => Self::LONGEST_ARRAY,
            "flat_unique" => Self::FLAT_UNIQUE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogsAggregationFunction {
    COUNT,
    CARDINALITY,
    PERCENTILE_75,
    PERCENTILE_90,
    PERCENTILE_95,
    PERCENTILE_98,
    PERCENTILE_99,
    SUM,
    MIN,
    MAX,
    AVG,
    MEDIAN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LogsAggregationFunction {
    fn to_string(&self) -> String {
        match self {
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::PERCENTILE_75 => String::from("pc75"),
            Self::PERCENTILE_90 => String::from("pc90"),
            Self::PERCENTILE_95 => String::from("pc95"),
            Self::PERCENTILE_98 => String::from("pc98"),
            Self::PERCENTILE_99 => String::from("pc99"),
            Self::SUM => String::from("sum"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::AVG => String::from("avg"),
            Self::MEDIAN => String::from("median"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LogsAggregationFunction {
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

impl<'de> Deserialize<'de> for LogsAggregationFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "count" => Self::COUNT,
            "cardinality" => Self::CARDINALITY,
            "pc75" => Self::PERCENTILE_75,
            "pc90" => Self::PERCENTILE_90,
            "pc95" => Self::PERCENTILE_95,
            "pc98" => Self::PERCENTILE_98,
            "pc99" => Self::PERCENTILE_99,
            "sum" => Self::SUM,
            "min" => Self::MIN,
            "max" => Self::MAX,
            "avg" => Self::AVG,
            "median" => Self::MEDIAN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

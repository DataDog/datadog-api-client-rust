// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventsAggregationValue {
    AVG,
    CARDINALITY,
    COUNT,
    DELTA,
    EARLIEST,
    LATEST,
    MAX,
    MEDIAN,
    MIN,
    MOST_FREQUENT,
    SUM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for EventsAggregationValue {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::COUNT => String::from("count"),
            Self::DELTA => String::from("delta"),
            Self::EARLIEST => String::from("earliest"),
            Self::LATEST => String::from("latest"),
            Self::MAX => String::from("max"),
            Self::MEDIAN => String::from("median"),
            Self::MIN => String::from("min"),
            Self::MOST_FREQUENT => String::from("most_frequent"),
            Self::SUM => String::from("sum"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for EventsAggregationValue {
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

impl<'de> Deserialize<'de> for EventsAggregationValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => Self::AVG,
            "cardinality" => Self::CARDINALITY,
            "count" => Self::COUNT,
            "delta" => Self::DELTA,
            "earliest" => Self::EARLIEST,
            "latest" => Self::LATEST,
            "max" => Self::MAX,
            "median" => Self::MEDIAN,
            "min" => Self::MIN,
            "most_frequent" => Self::MOST_FREQUENT,
            "sum" => Self::SUM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

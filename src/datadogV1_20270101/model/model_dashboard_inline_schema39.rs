// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardInlineSchema39 {
	SUM,
	MIN,
	MAX,
	AVG,
	COUNT,
	CARDINALITY,
	MEDIAN,
	DELTA,
	LATEST,
	EARLIEST,
	MOST_FREQUENT,
	ANY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DashboardInlineSchema39 {
    fn to_string(&self) -> String {
        match self {
            Self::SUM => String::from("sum"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::AVG => String::from("avg"),
            Self::COUNT => String::from("count"),
            Self::CARDINALITY => String::from("cardinality"),
            Self::MEDIAN => String::from("median"),
            Self::DELTA => String::from("delta"),
            Self::LATEST => String::from("latest"),
            Self::EARLIEST => String::from("earliest"),
            Self::MOST_FREQUENT => String::from("most_frequent"),
            Self::ANY => String::from("any"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DashboardInlineSchema39 {
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

impl<'de> Deserialize<'de> for DashboardInlineSchema39 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"sum" => Self::SUM,"min" => Self::MIN,"max" => Self::MAX,"avg" => Self::AVG,"count" => Self::COUNT,"cardinality" => Self::CARDINALITY,"median" => Self::MEDIAN,"delta" => Self::DELTA,"latest" => Self::LATEST,"earliest" => Self::EARLIEST,"most_frequent" => Self::MOST_FREQUENT,"any" => Self::ANY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

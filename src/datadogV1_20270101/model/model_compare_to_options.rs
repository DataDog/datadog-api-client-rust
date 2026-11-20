// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompareToOptions {
	HOUR_BEFORE,
	DAY_BEFORE,
	WEEK_BEFORE,
	MONTH_BEFORE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CompareToOptions {
    fn to_string(&self) -> String {
        match self {
            Self::HOUR_BEFORE => String::from("hour_before"),
            Self::DAY_BEFORE => String::from("day_before"),
            Self::WEEK_BEFORE => String::from("week_before"),
            Self::MONTH_BEFORE => String::from("month_before"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CompareToOptions {
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

impl<'de> Deserialize<'de> for CompareToOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"hour_before" => Self::HOUR_BEFORE,"day_before" => Self::DAY_BEFORE,"week_before" => Self::WEEK_BEFORE,"month_before" => Self::MONTH_BEFORE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

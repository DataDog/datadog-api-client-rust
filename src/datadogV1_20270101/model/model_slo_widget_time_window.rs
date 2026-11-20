// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SloWidgetTimeWindow {
	WEEK_TO_DATE,
	MONTH_TO_DATE,
	SEVEN_D,
	THIRTY_D,
	NINETY_D,
	PREVIOUS_WEEK,
	PREVIOUS_MONTH,
	GLOBAL_TIME,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SloWidgetTimeWindow {
    fn to_string(&self) -> String {
        match self {
            Self::WEEK_TO_DATE => String::from("week_to_date"),
            Self::MONTH_TO_DATE => String::from("month_to_date"),
            Self::SEVEN_D => String::from("7d"),
            Self::THIRTY_D => String::from("30d"),
            Self::NINETY_D => String::from("90d"),
            Self::PREVIOUS_WEEK => String::from("previous_week"),
            Self::PREVIOUS_MONTH => String::from("previous_month"),
            Self::GLOBAL_TIME => String::from("global_time"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SloWidgetTimeWindow {
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

impl<'de> Deserialize<'de> for SloWidgetTimeWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"week_to_date" => Self::WEEK_TO_DATE,"month_to_date" => Self::MONTH_TO_DATE,"7d" => Self::SEVEN_D,"30d" => Self::THIRTY_D,"90d" => Self::NINETY_D,"previous_week" => Self::PREVIOUS_WEEK,"previous_month" => Self::PREVIOUS_MONTH,"global_time" => Self::GLOBAL_TIME,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

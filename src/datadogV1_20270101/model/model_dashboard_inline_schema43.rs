// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardInlineSchema43 {
	GLOBAL,
	ALERT,
	FULL_WEEK,
	FULL_MONTH,
	WEEK_TO_DATE,
	MONTH_TO_DATE,
	YEAR_TO_DATE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DashboardInlineSchema43 {
    fn to_string(&self) -> String {
        match self {
            Self::GLOBAL => String::from("global"),
            Self::ALERT => String::from("alert"),
            Self::FULL_WEEK => String::from("full_week"),
            Self::FULL_MONTH => String::from("full_month"),
            Self::WEEK_TO_DATE => String::from("week_to_date"),
            Self::MONTH_TO_DATE => String::from("month_to_date"),
            Self::YEAR_TO_DATE => String::from("year_to_date"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DashboardInlineSchema43 {
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

impl<'de> Deserialize<'de> for DashboardInlineSchema43 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"global" => Self::GLOBAL,"alert" => Self::ALERT,"full_week" => Self::FULL_WEEK,"full_month" => Self::FULL_MONTH,"week_to_date" => Self::WEEK_TO_DATE,"month_to_date" => Self::MONTH_TO_DATE,"year_to_date" => Self::YEAR_TO_DATE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

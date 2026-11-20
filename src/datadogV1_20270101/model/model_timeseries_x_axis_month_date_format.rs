// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeseriesXAxisMonthDateFormat {
	AUTO,
	MMM,
	MMM_YYYY,
	MM_YYYY,
	MMMM_YYYY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TimeseriesXAxisMonthDateFormat {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::MMM => String::from("MMM"),
            Self::MMM_YYYY => String::from("MMM yyyy"),
            Self::MM_YYYY => String::from("MM/yyyy"),
            Self::MMMM_YYYY => String::from("MMMM yyyy"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TimeseriesXAxisMonthDateFormat {
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

impl<'de> Deserialize<'de> for TimeseriesXAxisMonthDateFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"auto" => Self::AUTO,"MMM" => Self::MMM,"MMM yyyy" => Self::MMM_YYYY,"MM/yyyy" => Self::MM_YYYY,"MMMM yyyy" => Self::MMMM_YYYY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

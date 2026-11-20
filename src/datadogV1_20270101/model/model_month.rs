// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Month {
	JANUARY,
	FEBRUARY,
	MARCH,
	APRIL,
	MAY,
	JUNE,
	JULY,
	AUGUST,
	SEPTEMBER,
	OCTOBER,
	NOVEMBER,
	DECEMBER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for Month {
    fn to_string(&self) -> String {
        match self {
            Self::JANUARY => String::from("january"),
            Self::FEBRUARY => String::from("february"),
            Self::MARCH => String::from("march"),
            Self::APRIL => String::from("april"),
            Self::MAY => String::from("may"),
            Self::JUNE => String::from("june"),
            Self::JULY => String::from("july"),
            Self::AUGUST => String::from("august"),
            Self::SEPTEMBER => String::from("september"),
            Self::OCTOBER => String::from("october"),
            Self::NOVEMBER => String::from("november"),
            Self::DECEMBER => String::from("december"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for Month {
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

impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"january" => Self::JANUARY,"february" => Self::FEBRUARY,"march" => Self::MARCH,"april" => Self::APRIL,"may" => Self::MAY,"june" => Self::JUNE,"july" => Self::JULY,"august" => Self::AUGUST,"september" => Self::SEPTEMBER,"october" => Self::OCTOBER,"november" => Self::NOVEMBER,"december" => Self::DECEMBER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

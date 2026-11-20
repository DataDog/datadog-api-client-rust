// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Hour12Format {
	TWELVE_AM,
	ONE_AM,
	TWO_AM,
	THREE_AM,
	FOUR_AM,
	FIVE_AM,
	SIX_AM,
	SEVEN_AM,
	EIGHT_AM,
	NINE_AM,
	TEN_AM,
	ELEVEN_AM,
	TWELVE_PM,
	ONE_PM,
	TWO_PM,
	THREE_PM,
	FOUR_PM,
	FIVE_PM,
	SIX_PM,
	SEVEN_PM,
	EIGHT_PM,
	NINE_PM,
	TEN_PM,
	ELEVEN_PM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for Hour12Format {
    fn to_string(&self) -> String {
        match self {
            Self::TWELVE_AM => String::from("12am"),
            Self::ONE_AM => String::from("1am"),
            Self::TWO_AM => String::from("2am"),
            Self::THREE_AM => String::from("3am"),
            Self::FOUR_AM => String::from("4am"),
            Self::FIVE_AM => String::from("5am"),
            Self::SIX_AM => String::from("6am"),
            Self::SEVEN_AM => String::from("7am"),
            Self::EIGHT_AM => String::from("8am"),
            Self::NINE_AM => String::from("9am"),
            Self::TEN_AM => String::from("10am"),
            Self::ELEVEN_AM => String::from("11am"),
            Self::TWELVE_PM => String::from("12pm"),
            Self::ONE_PM => String::from("1pm"),
            Self::TWO_PM => String::from("2pm"),
            Self::THREE_PM => String::from("3pm"),
            Self::FOUR_PM => String::from("4pm"),
            Self::FIVE_PM => String::from("5pm"),
            Self::SIX_PM => String::from("6pm"),
            Self::SEVEN_PM => String::from("7pm"),
            Self::EIGHT_PM => String::from("8pm"),
            Self::NINE_PM => String::from("9pm"),
            Self::TEN_PM => String::from("10pm"),
            Self::ELEVEN_PM => String::from("11pm"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for Hour12Format {
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

impl<'de> Deserialize<'de> for Hour12Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"12am" => Self::TWELVE_AM,"1am" => Self::ONE_AM,"2am" => Self::TWO_AM,"3am" => Self::THREE_AM,"4am" => Self::FOUR_AM,"5am" => Self::FIVE_AM,"6am" => Self::SIX_AM,"7am" => Self::SEVEN_AM,"8am" => Self::EIGHT_AM,"9am" => Self::NINE_AM,"10am" => Self::TEN_AM,"11am" => Self::ELEVEN_AM,"12pm" => Self::TWELVE_PM,"1pm" => Self::ONE_PM,"2pm" => Self::TWO_PM,"3pm" => Self::THREE_PM,"4pm" => Self::FOUR_PM,"5pm" => Self::FIVE_PM,"6pm" => Self::SIX_PM,"7pm" => Self::SEVEN_PM,"8pm" => Self::EIGHT_PM,"9pm" => Self::NINE_PM,"10pm" => Self::TEN_PM,"11pm" => Self::ELEVEN_PM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

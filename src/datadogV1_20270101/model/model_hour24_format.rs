// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Hour24Format {
	ZERO,
	ONE,
	TWO,
	THREE,
	FOUR,
	FIVE,
	SIX,
	SEVEN,
	EIGHT,
	NINE,
	TEN,
	ELEVEN,
	TWELVE,
	THIRTEEN,
	FOURTEEN,
	FIFTEEN,
	SIXTEEN,
	SEVENTEEN,
	EIGHTEEN,
	NINETEEN,
	TWENTY,
	TWENTY_ONE,
	TWENTY_TWO,
	TWENTY_THREE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for Hour24Format {
    fn to_string(&self) -> String {
        match self {
            Self::ZERO => String::from("0"),
            Self::ONE => String::from("1"),
            Self::TWO => String::from("2"),
            Self::THREE => String::from("3"),
            Self::FOUR => String::from("4"),
            Self::FIVE => String::from("5"),
            Self::SIX => String::from("6"),
            Self::SEVEN => String::from("7"),
            Self::EIGHT => String::from("8"),
            Self::NINE => String::from("9"),
            Self::TEN => String::from("10"),
            Self::ELEVEN => String::from("11"),
            Self::TWELVE => String::from("12"),
            Self::THIRTEEN => String::from("13"),
            Self::FOURTEEN => String::from("14"),
            Self::FIFTEEN => String::from("15"),
            Self::SIXTEEN => String::from("16"),
            Self::SEVENTEEN => String::from("17"),
            Self::EIGHTEEN => String::from("18"),
            Self::NINETEEN => String::from("19"),
            Self::TWENTY => String::from("20"),
            Self::TWENTY_ONE => String::from("21"),
            Self::TWENTY_TWO => String::from("22"),
            Self::TWENTY_THREE => String::from("23"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for Hour24Format {
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

impl<'de> Deserialize<'de> for Hour24Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"0" => Self::ZERO,"1" => Self::ONE,"2" => Self::TWO,"3" => Self::THREE,"4" => Self::FOUR,"5" => Self::FIVE,"6" => Self::SIX,"7" => Self::SEVEN,"8" => Self::EIGHT,"9" => Self::NINE,"10" => Self::TEN,"11" => Self::ELEVEN,"12" => Self::TWELVE,"13" => Self::THIRTEEN,"14" => Self::FOURTEEN,"15" => Self::FIFTEEN,"16" => Self::SIXTEEN,"17" => Self::SEVENTEEN,"18" => Self::EIGHTEEN,"19" => Self::NINETEEN,"20" => Self::TWENTY,"21" => Self::TWENTY_ONE,"22" => Self::TWENTY_TWO,"23" => Self::TWENTY_THREE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

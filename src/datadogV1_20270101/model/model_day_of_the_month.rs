// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DayOfTheMonth {
	ONE_ST,
	TWO_ND,
	THREE_RD,
	FOUR_TH,
	FIVE_TH,
	SIX_TH,
	SEVEN_TH,
	EIGHT_TH,
	NINE_TH,
	TEN_TH,
	ELEVEN_TH,
	TWELVE_TH,
	THIRTEEN_TH,
	FOURTEEN_TH,
	FIFTEEN_TH,
	SIXTEEN_TH,
	SEVENTEEN_TH,
	EIGHTEEN_TH,
	NINETEEN_TH,
	TWENTY_TH,
	TWENTY_ONE_ST,
	TWENTY_TWO_ND,
	TWENTY_THREE_RD,
	TWENTY_FOUR_TH,
	TWENTY_FIVE_TH,
	TWENTY_SIX_TH,
	TWENTY_SEVEN_TH,
	TWENTY_EIGHT_TH,
	TWENTY_NINE_TH,
	THIRTY_TH,
	THIRTY_ONE_ST,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DayOfTheMonth {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_ST => String::from("1st"),
            Self::TWO_ND => String::from("2nd"),
            Self::THREE_RD => String::from("3rd"),
            Self::FOUR_TH => String::from("4th"),
            Self::FIVE_TH => String::from("5th"),
            Self::SIX_TH => String::from("6th"),
            Self::SEVEN_TH => String::from("7th"),
            Self::EIGHT_TH => String::from("8th"),
            Self::NINE_TH => String::from("9th"),
            Self::TEN_TH => String::from("10th"),
            Self::ELEVEN_TH => String::from("11th"),
            Self::TWELVE_TH => String::from("12th"),
            Self::THIRTEEN_TH => String::from("13th"),
            Self::FOURTEEN_TH => String::from("14th"),
            Self::FIFTEEN_TH => String::from("15th"),
            Self::SIXTEEN_TH => String::from("16th"),
            Self::SEVENTEEN_TH => String::from("17th"),
            Self::EIGHTEEN_TH => String::from("18th"),
            Self::NINETEEN_TH => String::from("19th"),
            Self::TWENTY_TH => String::from("20th"),
            Self::TWENTY_ONE_ST => String::from("21st"),
            Self::TWENTY_TWO_ND => String::from("22nd"),
            Self::TWENTY_THREE_RD => String::from("23rd"),
            Self::TWENTY_FOUR_TH => String::from("24th"),
            Self::TWENTY_FIVE_TH => String::from("25th"),
            Self::TWENTY_SIX_TH => String::from("26th"),
            Self::TWENTY_SEVEN_TH => String::from("27th"),
            Self::TWENTY_EIGHT_TH => String::from("28th"),
            Self::TWENTY_NINE_TH => String::from("29th"),
            Self::THIRTY_TH => String::from("30th"),
            Self::THIRTY_ONE_ST => String::from("31st"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DayOfTheMonth {
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

impl<'de> Deserialize<'de> for DayOfTheMonth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"1st" => Self::ONE_ST,"2nd" => Self::TWO_ND,"3rd" => Self::THREE_RD,"4th" => Self::FOUR_TH,"5th" => Self::FIVE_TH,"6th" => Self::SIX_TH,"7th" => Self::SEVEN_TH,"8th" => Self::EIGHT_TH,"9th" => Self::NINE_TH,"10th" => Self::TEN_TH,"11th" => Self::ELEVEN_TH,"12th" => Self::TWELVE_TH,"13th" => Self::THIRTEEN_TH,"14th" => Self::FOURTEEN_TH,"15th" => Self::FIFTEEN_TH,"16th" => Self::SIXTEEN_TH,"17th" => Self::SEVENTEEN_TH,"18th" => Self::EIGHTEEN_TH,"19th" => Self::NINETEEN_TH,"20th" => Self::TWENTY_TH,"21st" => Self::TWENTY_ONE_ST,"22nd" => Self::TWENTY_TWO_ND,"23rd" => Self::TWENTY_THREE_RD,"24th" => Self::TWENTY_FOUR_TH,"25th" => Self::TWENTY_FIVE_TH,"26th" => Self::TWENTY_SIX_TH,"27th" => Self::TWENTY_SEVEN_TH,"28th" => Self::TWENTY_EIGHT_TH,"29th" => Self::TWENTY_NINE_TH,"30th" => Self::THIRTY_TH,"31st" => Self::THIRTY_ONE_ST,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeSpan {
	ONE_M,
	FIVE_M,
	TEN_M,
	FIFTEEN_M,
	THIRTY_M,
	ONE_H,
	FOUR_H,
	ONE_D,
	TWO_D,
	ONE_W,
	ONE_MO,
	THREE_MO,
	SIX_MO,
	ONE_Y,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TimeSpan {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_M => String::from("1m"),
            Self::FIVE_M => String::from("5m"),
            Self::TEN_M => String::from("10m"),
            Self::FIFTEEN_M => String::from("15m"),
            Self::THIRTY_M => String::from("30m"),
            Self::ONE_H => String::from("1h"),
            Self::FOUR_H => String::from("4h"),
            Self::ONE_D => String::from("1d"),
            Self::TWO_D => String::from("2d"),
            Self::ONE_W => String::from("1w"),
            Self::ONE_MO => String::from("1mo"),
            Self::THREE_MO => String::from("3mo"),
            Self::SIX_MO => String::from("6mo"),
            Self::ONE_Y => String::from("1y"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TimeSpan {
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

impl<'de> Deserialize<'de> for TimeSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"1m" => Self::ONE_M,"5m" => Self::FIVE_M,"10m" => Self::TEN_M,"15m" => Self::FIFTEEN_M,"30m" => Self::THIRTY_M,"1h" => Self::ONE_H,"4h" => Self::FOUR_H,"1d" => Self::ONE_D,"2d" => Self::TWO_D,"1w" => Self::ONE_W,"1mo" => Self::ONE_MO,"3mo" => Self::THREE_MO,"6mo" => Self::SIX_MO,"1y" => Self::ONE_Y,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayRateLimitWindowLimitWindow {
	FIVE_MINUTES,
	FIFTEEN_MINUTES,
	THIRTY_MINUTES,
	ONE_HOUR,
	THREE_HOURS,
	SIX_HOURS,
	TWELVE_HOURS,
	TWENTY_FOUR_HOURS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayRateLimitWindowLimitWindow {
    fn to_string(&self) -> String {
        match self {
            Self::FIVE_MINUTES => String::from("5m"),
            Self::FIFTEEN_MINUTES => String::from("15m"),
            Self::THIRTY_MINUTES => String::from("30m"),
            Self::ONE_HOUR => String::from("1h"),
            Self::THREE_HOURS => String::from("3h"),
            Self::SIX_HOURS => String::from("6h"),
            Self::TWELVE_HOURS => String::from("12h"),
            Self::TWENTY_FOUR_HOURS => String::from("24h"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayRateLimitWindowLimitWindow {
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitWindowLimitWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"5m" => Self::FIVE_MINUTES,"15m" => Self::FIFTEEN_MINUTES,"30m" => Self::THIRTY_MINUTES,"1h" => Self::ONE_HOUR,"3h" => Self::THREE_HOURS,"6h" => Self::SIX_HOURS,"12h" => Self::TWELVE_HOURS,"24h" => Self::TWENTY_FOUR_HOURS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

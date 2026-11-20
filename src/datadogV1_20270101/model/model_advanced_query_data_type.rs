// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdvancedQueryDataType {
	STRING,
	GROUP,
	INET,
	INT64,
	FLOAT64,
	TIMESTAMP,
	DATE,
	INTERVAL,
	BOOL,
	JSON,
	HSTORE,
	ARRAYLTSTRINGGT,
	ARRAYLTINT64GT,
	ARRAYLTFLOAT64GT,
	ARRAYLTTIMESTAMPGT,
	ARRAYLTINTERVALGT,
	ARRAYLTBOOLGT,
	ARRAYLTJSONGT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AdvancedQueryDataType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING => String::from("string"),
            Self::GROUP => String::from("group"),
            Self::INET => String::from("inet"),
            Self::INT64 => String::from("int64"),
            Self::FLOAT64 => String::from("float64"),
            Self::TIMESTAMP => String::from("timestamp"),
            Self::DATE => String::from("date"),
            Self::INTERVAL => String::from("interval"),
            Self::BOOL => String::from("bool"),
            Self::JSON => String::from("json"),
            Self::HSTORE => String::from("hstore"),
            Self::ARRAYLTSTRINGGT => String::from("array<string>"),
            Self::ARRAYLTINT64GT => String::from("array<int64>"),
            Self::ARRAYLTFLOAT64GT => String::from("array<float64>"),
            Self::ARRAYLTTIMESTAMPGT => String::from("array<timestamp>"),
            Self::ARRAYLTINTERVALGT => String::from("array<interval>"),
            Self::ARRAYLTBOOLGT => String::from("array<bool>"),
            Self::ARRAYLTJSONGT => String::from("array<json>"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AdvancedQueryDataType {
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

impl<'de> Deserialize<'de> for AdvancedQueryDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"string" => Self::STRING,"group" => Self::GROUP,"inet" => Self::INET,"int64" => Self::INT64,"float64" => Self::FLOAT64,"timestamp" => Self::TIMESTAMP,"date" => Self::DATE,"interval" => Self::INTERVAL,"bool" => Self::BOOL,"json" => Self::JSON,"hstore" => Self::HSTORE,"array<string>" => Self::ARRAYLTSTRINGGT,"array<int64>" => Self::ARRAYLTINT64GT,"array<float64>" => Self::ARRAYLTFLOAT64GT,"array<timestamp>" => Self::ARRAYLTTIMESTAMPGT,"array<interval>" => Self::ARRAYLTINTERVALGT,"array<bool>" => Self::ARRAYLTBOOLGT,"array<json>" => Self::ARRAYLTJSONGT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

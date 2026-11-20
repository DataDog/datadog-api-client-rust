// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseMetric {
	TIME_TO_CLOSE,
	TIME_TO_ASSIGN,
	EVENT_COUNT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CaseMetric {
    fn to_string(&self) -> String {
        match self {
            Self::TIME_TO_CLOSE => String::from("time_to_close"),
            Self::TIME_TO_ASSIGN => String::from("time_to_assign"),
            Self::EVENT_COUNT => String::from("event_count"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CaseMetric {
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

impl<'de> Deserialize<'de> for CaseMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"time_to_close" => Self::TIME_TO_CLOSE,"time_to_assign" => Self::TIME_TO_ASSIGN,"event_count" => Self::EVENT_COUNT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

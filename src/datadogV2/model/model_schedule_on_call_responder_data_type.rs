// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScheduleOnCallResponderDataType {
    SCHEDULE_ONCALL_RESPONDER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ScheduleOnCallResponderDataType {
    fn to_string(&self) -> String {
        match self {
            Self::SCHEDULE_ONCALL_RESPONDER => String::from("schedule_oncall_responder"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ScheduleOnCallResponderDataType {
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

impl<'de> Deserialize<'de> for ScheduleOnCallResponderDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "schedule_oncall_responder" => Self::SCHEDULE_ONCALL_RESPONDER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

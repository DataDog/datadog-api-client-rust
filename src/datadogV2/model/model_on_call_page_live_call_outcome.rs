// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPageLiveCallOutcome {
    SUCCESS_CALL_ANSWERED,
    FAILURE_NO_RESPONDER,
    FAILURE_CALLER_ABANDONED,
    FAILURE_OUTSIDE_SUPPORT_HOURS,
    FAILURE_INTERNAL_ERROR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPageLiveCallOutcome {
    fn to_string(&self) -> String {
        match self {
            Self::SUCCESS_CALL_ANSWERED => String::from("success_call_answered"),
            Self::FAILURE_NO_RESPONDER => String::from("failure_no_responder"),
            Self::FAILURE_CALLER_ABANDONED => String::from("failure_caller_abandoned"),
            Self::FAILURE_OUTSIDE_SUPPORT_HOURS => String::from("failure_outside_support_hours"),
            Self::FAILURE_INTERNAL_ERROR => String::from("failure_internal_error"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPageLiveCallOutcome {
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

impl<'de> Deserialize<'de> for OnCallPageLiveCallOutcome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "success_call_answered" => Self::SUCCESS_CALL_ANSWERED,
            "failure_no_responder" => Self::FAILURE_NO_RESPONDER,
            "failure_caller_abandoned" => Self::FAILURE_CALLER_ABANDONED,
            "failure_outside_support_hours" => Self::FAILURE_OUTSIDE_SUPPORT_HOURS,
            "failure_internal_error" => Self::FAILURE_INTERNAL_ERROR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

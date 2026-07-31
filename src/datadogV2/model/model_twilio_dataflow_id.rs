// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TwilioDataflowId {
    CLOUD_COST_METRICS,
    EVENTS_LOGS,
    MESSAGES_LOGS,
    ALERTS_LOGS,
    CALL_SUMMARIES_LOGS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TwilioDataflowId {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUD_COST_METRICS => String::from("twilio-cloud-cost-metrics"),
            Self::EVENTS_LOGS => String::from("twilio-events-logs"),
            Self::MESSAGES_LOGS => String::from("twilio-messages-logs"),
            Self::ALERTS_LOGS => String::from("twilio-alerts-logs"),
            Self::CALL_SUMMARIES_LOGS => String::from("twilio-call-summaries-logs"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TwilioDataflowId {
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

impl<'de> Deserialize<'de> for TwilioDataflowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "twilio-cloud-cost-metrics" => Self::CLOUD_COST_METRICS,
            "twilio-events-logs" => Self::EVENTS_LOGS,
            "twilio-messages-logs" => Self::MESSAGES_LOGS,
            "twilio-alerts-logs" => Self::ALERTS_LOGS,
            "twilio-call-summaries-logs" => Self::CALL_SUMMARIES_LOGS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutomationRuleTriggerType {
    CASE_CREATED,
    STATUS_TRANSITIONED,
    ATTRIBUTE_VALUE_CHANGED,
    EVENT_CORRELATION_SIGNAL_CORRELATED,
    CASE_REVIEW_APPROVED,
    COMMENT_ADDED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AutomationRuleTriggerType {
    fn to_string(&self) -> String {
        match self {
            Self::CASE_CREATED => String::from("case_created"),
            Self::STATUS_TRANSITIONED => String::from("status_transitioned"),
            Self::ATTRIBUTE_VALUE_CHANGED => String::from("attribute_value_changed"),
            Self::EVENT_CORRELATION_SIGNAL_CORRELATED => {
                String::from("event_correlation_signal_correlated")
            }
            Self::CASE_REVIEW_APPROVED => String::from("case_review_approved"),
            Self::COMMENT_ADDED => String::from("comment_added"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AutomationRuleTriggerType {
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

impl<'de> Deserialize<'de> for AutomationRuleTriggerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "case_created" => Self::CASE_CREATED,
            "status_transitioned" => Self::STATUS_TRANSITIONED,
            "attribute_value_changed" => Self::ATTRIBUTE_VALUE_CHANGED,
            "event_correlation_signal_correlated" => Self::EVENT_CORRELATION_SIGNAL_CORRELATED,
            "case_review_approved" => Self::CASE_REVIEW_APPROVED,
            "comment_added" => Self::COMMENT_ADDED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

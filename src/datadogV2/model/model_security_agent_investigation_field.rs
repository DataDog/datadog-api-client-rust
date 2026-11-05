// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityAgentInvestigationField {
    SIGNAL_ID,
    RULE_ID,
    STATE,
    VERDICT,
    CONCLUSION,
    SIGNAL,
    KEY_ENTITIES,
    KEY_EVIDENCE,
    STEPS,
    RESPONSE_SUGGESTIONS,
    CREATED_AT,
    UPDATED_AT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityAgentInvestigationField {
    fn to_string(&self) -> String {
        match self {
            Self::SIGNAL_ID => String::from("signal_id"),
            Self::RULE_ID => String::from("rule_id"),
            Self::STATE => String::from("state"),
            Self::VERDICT => String::from("verdict"),
            Self::CONCLUSION => String::from("conclusion"),
            Self::SIGNAL => String::from("signal"),
            Self::KEY_ENTITIES => String::from("key_entities"),
            Self::KEY_EVIDENCE => String::from("key_evidence"),
            Self::STEPS => String::from("steps"),
            Self::RESPONSE_SUGGESTIONS => String::from("response_suggestions"),
            Self::CREATED_AT => String::from("created_at"),
            Self::UPDATED_AT => String::from("updated_at"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityAgentInvestigationField {
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "signal_id" => Self::SIGNAL_ID,
            "rule_id" => Self::RULE_ID,
            "state" => Self::STATE,
            "verdict" => Self::VERDICT,
            "conclusion" => Self::CONCLUSION,
            "signal" => Self::SIGNAL,
            "key_entities" => Self::KEY_ENTITIES,
            "key_evidence" => Self::KEY_EVIDENCE,
            "steps" => Self::STEPS,
            "response_suggestions" => Self::RESPONSE_SUGGESTIONS,
            "created_at" => Self::CREATED_AT,
            "updated_at" => Self::UPDATED_AT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

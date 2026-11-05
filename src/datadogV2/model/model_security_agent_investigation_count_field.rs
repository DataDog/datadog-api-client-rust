// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityAgentInvestigationCountField {
    VERDICT,
    RULE_TYPE,
    SIGNAL_SOURCE,
    ENTITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityAgentInvestigationCountField {
    fn to_string(&self) -> String {
        match self {
            Self::VERDICT => String::from("verdict"),
            Self::RULE_TYPE => String::from("rule_type"),
            Self::SIGNAL_SOURCE => String::from("signal_source"),
            Self::ENTITY => String::from("entity"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityAgentInvestigationCountField {
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationCountField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "verdict" => Self::VERDICT,
            "rule_type" => Self::RULE_TYPE,
            "signal_source" => Self::SIGNAL_SOURCE,
            "entity" => Self::ENTITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

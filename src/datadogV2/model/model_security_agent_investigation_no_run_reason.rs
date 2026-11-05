// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityAgentInvestigationNoRunReason {
    INVESTIGATION_NOT_STARTED,
    NOT_INVESTIGATED,
    RULE_NOT_ENABLED,
    SIGNAL_NOT_FOUND,
    ORG_NOT_ACTIVATED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityAgentInvestigationNoRunReason {
    fn to_string(&self) -> String {
        match self {
            Self::INVESTIGATION_NOT_STARTED => String::from("investigation_not_started"),
            Self::NOT_INVESTIGATED => String::from("not_investigated"),
            Self::RULE_NOT_ENABLED => String::from("rule_not_enabled"),
            Self::SIGNAL_NOT_FOUND => String::from("signal_not_found"),
            Self::ORG_NOT_ACTIVATED => String::from("org_not_activated"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityAgentInvestigationNoRunReason {
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationNoRunReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "investigation_not_started" => Self::INVESTIGATION_NOT_STARTED,
            "not_investigated" => Self::NOT_INVESTIGATED,
            "rule_not_enabled" => Self::RULE_NOT_ENABLED,
            "signal_not_found" => Self::SIGNAL_NOT_FOUND,
            "org_not_activated" => Self::ORG_NOT_ACTIVATED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionEventsDataSource {
    LOGS,
    SPANS,
    NETWORK,
    RUM,
    SECURITY_SIGNALS,
    PROFILES,
    AUDIT,
    EVENTS,
    CI_TESTS,
    CI_PIPELINES,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for FormulaAndFunctionEventsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::SPANS => String::from("spans"),
            Self::NETWORK => String::from("network"),
            Self::RUM => String::from("rum"),
            Self::SECURITY_SIGNALS => String::from("security_signals"),
            Self::PROFILES => String::from("profiles"),
            Self::AUDIT => String::from("audit"),
            Self::EVENTS => String::from("events"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormulaAndFunctionEventsDataSource {
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

impl<'de> Deserialize<'de> for FormulaAndFunctionEventsDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "logs" => Self::LOGS,
            "spans" => Self::SPANS,
            "network" => Self::NETWORK,
            "rum" => Self::RUM,
            "security_signals" => Self::SECURITY_SIGNALS,
            "profiles" => Self::PROFILES,
            "audit" => Self::AUDIT,
            "events" => Self::EVENTS,
            "ci_tests" => Self::CI_TESTS,
            "ci_pipelines" => Self::CI_PIPELINES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

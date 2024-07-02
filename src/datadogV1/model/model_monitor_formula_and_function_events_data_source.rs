// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonitorFormulaAndFunctionEventsDataSource {
    RUM,
    CI_PIPELINES,
    CI_TESTS,
    AUDIT,
    EVENTS,
    LOGS,
    SPANS,
    DATABASE_QUERIES,
    NETWORK_PERFORMANCE_QUERIES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for MonitorFormulaAndFunctionEventsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::RUM => String::from("rum"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::AUDIT => String::from("audit"),
            Self::EVENTS => String::from("events"),
            Self::LOGS => String::from("logs"),
            Self::SPANS => String::from("spans"),
            Self::DATABASE_QUERIES => String::from("database_queries"),
            Self::NETWORK_PERFORMANCE_QUERIES => String::from("network_performance_queries"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonitorFormulaAndFunctionEventsDataSource {
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

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionEventsDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rum" => Self::RUM,
            "ci_pipelines" => Self::CI_PIPELINES,
            "ci_tests" => Self::CI_TESTS,
            "audit" => Self::AUDIT,
            "events" => Self::EVENTS,
            "logs" => Self::LOGS,
            "spans" => Self::SPANS,
            "database_queries" => Self::DATABASE_QUERIES,
            "network_performance_queries" => Self::NETWORK_PERFORMANCE_QUERIES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

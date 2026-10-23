// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrgConnectionTypeEnum {
    LOGS,
    METRICS,
    AUDIT,
    CI_PIPELINES,
    CI_TESTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OrgConnectionTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::METRICS => String::from("metrics"),
            Self::AUDIT => String::from("audit"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OrgConnectionTypeEnum {
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

impl<'de> Deserialize<'de> for OrgConnectionTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "logs" => Self::LOGS,
            "metrics" => Self::METRICS,
            "audit" => Self::AUDIT,
            "ci_pipelines" => Self::CI_PIPELINES,
            "ci_tests" => Self::CI_TESTS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

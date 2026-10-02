// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsExperimentDiagnosticsV2DTODataAttributesResult {
    PASS,
    FAIL,
    WARN,
    NO_DATA,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsExperimentDiagnosticsV2DTODataAttributesResult {
    fn to_string(&self) -> String {
        match self {
            Self::PASS => String::from("PASS"),
            Self::FAIL => String::from("FAIL"),
            Self::WARN => String::from("WARN"),
            Self::NO_DATA => String::from("NO_DATA"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsExperimentDiagnosticsV2DTODataAttributesResult {
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

impl<'de> Deserialize<'de> for ExperimentsExperimentDiagnosticsV2DTODataAttributesResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PASS" => Self::PASS,
            "FAIL" => Self::FAIL,
            "WARN" => Self::WARN,
            "NO_DATA" => Self::NO_DATA,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

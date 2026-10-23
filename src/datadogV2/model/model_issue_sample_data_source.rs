// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueSampleDataSource {
    APM,
    LOGS,
    RUM,
    LLMOBS,
    SYNTHETICS,
    NONE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueSampleDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::APM => String::from("APM"),
            Self::LOGS => String::from("LOGS"),
            Self::RUM => String::from("RUM"),
            Self::LLMOBS => String::from("LLMOBS"),
            Self::SYNTHETICS => String::from("SYNTHETICS"),
            Self::NONE => String::from("NONE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssueSampleDataSource {
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

impl<'de> Deserialize<'de> for IssueSampleDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "APM" => Self::APM,
            "LOGS" => Self::LOGS,
            "RUM" => Self::RUM,
            "LLMOBS" => Self::LLMOBS,
            "SYNTHETICS" => Self::SYNTHETICS,
            "NONE" => Self::NONE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

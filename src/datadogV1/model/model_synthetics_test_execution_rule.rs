// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestExecutionRule {
    BLOCKING,
    NON_BLOCKING,
    SKIPPED,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SyntheticsTestExecutionRule {
    fn to_string(&self) -> String {
        match self {
            Self::BLOCKING => String::from("blocking"),
            Self::NON_BLOCKING => String::from("non_blocking"),
            Self::SKIPPED => String::from("skipped"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsTestExecutionRule {
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

impl<'de> Deserialize<'de> for SyntheticsTestExecutionRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "blocking" => Self::BLOCKING,
            "non_blocking" => Self::NON_BLOCKING,
            "skipped" => Self::SKIPPED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

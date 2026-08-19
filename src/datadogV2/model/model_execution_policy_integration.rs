// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionPolicyIntegration {
    INTEGRATION_KUBERNETES,
    INTEGRATION_SCRIPT,
    INTEGRATION_REMOTE_ACTION,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExecutionPolicyIntegration {
    fn to_string(&self) -> String {
        match self {
            Self::INTEGRATION_KUBERNETES => String::from("INTEGRATION_KUBERNETES"),
            Self::INTEGRATION_SCRIPT => String::from("INTEGRATION_SCRIPT"),
            Self::INTEGRATION_REMOTE_ACTION => String::from("INTEGRATION_REMOTE_ACTION"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExecutionPolicyIntegration {
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

impl<'de> Deserialize<'de> for ExecutionPolicyIntegration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "INTEGRATION_KUBERNETES" => Self::INTEGRATION_KUBERNETES,
            "INTEGRATION_SCRIPT" => Self::INTEGRATION_SCRIPT,
            "INTEGRATION_REMOTE_ACTION" => Self::INTEGRATION_REMOTE_ACTION,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

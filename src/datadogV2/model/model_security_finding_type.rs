// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityFindingType {
    API_SECURITY,
    ATTACK_PATH,
    HOST_AND_CONTAINER_VULNERABILITY,
    IAC_MISCONFIGURATION,
    IDENTITY_RISK,
    LIBRARY_VULNERABILITY,
    MISCONFIGURATION,
    RUNTIME_CODE_VULNERABILITY,
    SECRET,
    STATIC_CODE_VULNERABILITY,
    WORKLOAD_ACTIVITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityFindingType {
    fn to_string(&self) -> String {
        match self {
            Self::API_SECURITY => String::from("api_security"),
            Self::ATTACK_PATH => String::from("attack_path"),
            Self::HOST_AND_CONTAINER_VULNERABILITY => {
                String::from("host_and_container_vulnerability")
            }
            Self::IAC_MISCONFIGURATION => String::from("iac_misconfiguration"),
            Self::IDENTITY_RISK => String::from("identity_risk"),
            Self::LIBRARY_VULNERABILITY => String::from("library_vulnerability"),
            Self::MISCONFIGURATION => String::from("misconfiguration"),
            Self::RUNTIME_CODE_VULNERABILITY => String::from("runtime_code_vulnerability"),
            Self::SECRET => String::from("secret"),
            Self::STATIC_CODE_VULNERABILITY => String::from("static_code_vulnerability"),
            Self::WORKLOAD_ACTIVITY => String::from("workload_activity"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityFindingType {
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

impl<'de> Deserialize<'de> for SecurityFindingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "api_security" => Self::API_SECURITY,
            "attack_path" => Self::ATTACK_PATH,
            "host_and_container_vulnerability" => Self::HOST_AND_CONTAINER_VULNERABILITY,
            "iac_misconfiguration" => Self::IAC_MISCONFIGURATION,
            "identity_risk" => Self::IDENTITY_RISK,
            "library_vulnerability" => Self::LIBRARY_VULNERABILITY,
            "misconfiguration" => Self::MISCONFIGURATION,
            "runtime_code_vulnerability" => Self::RUNTIME_CODE_VULNERABILITY,
            "secret" => Self::SECRET,
            "static_code_vulnerability" => Self::STATIC_CODE_VULNERABILITY,
            "workload_activity" => Self::WORKLOAD_ACTIVITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleTypesItems {
    APPLICATION_CODE_VULNERABILITY,
    APPLICATION_LIBRARY_VULNERABILITY,
    ATTACK_PATH,
    CONTAINER_IMAGE_VULNERABILITY,
    HOST_VULNERABILITY,
    IDENTITY_RISK,
    MISCONFIGURATION,
    API_SECURITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RuleTypesItems {
    fn to_string(&self) -> String {
        match self {
            Self::APPLICATION_CODE_VULNERABILITY => String::from("application_code_vulnerability"),
            Self::APPLICATION_LIBRARY_VULNERABILITY => {
                String::from("application_library_vulnerability")
            }
            Self::ATTACK_PATH => String::from("attack_path"),
            Self::CONTAINER_IMAGE_VULNERABILITY => String::from("container_image_vulnerability"),
            Self::HOST_VULNERABILITY => String::from("host_vulnerability"),
            Self::IDENTITY_RISK => String::from("identity_risk"),
            Self::MISCONFIGURATION => String::from("misconfiguration"),
            Self::API_SECURITY => String::from("api_security"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RuleTypesItems {
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

impl<'de> Deserialize<'de> for RuleTypesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "application_code_vulnerability" => Self::APPLICATION_CODE_VULNERABILITY,
            "application_library_vulnerability" => Self::APPLICATION_LIBRARY_VULNERABILITY,
            "attack_path" => Self::ATTACK_PATH,
            "container_image_vulnerability" => Self::CONTAINER_IMAGE_VULNERABILITY,
            "host_vulnerability" => Self::HOST_VULNERABILITY,
            "identity_risk" => Self::IDENTITY_RISK,
            "misconfiguration" => Self::MISCONFIGURATION,
            "api_security" => Self::API_SECURITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

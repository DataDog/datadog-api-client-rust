// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerraformCloudAuthPersonaMappingType {
	TERRAFORM_CLOUD_AUTH_CONFIG,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TerraformCloudAuthPersonaMappingType {
    fn to_string(&self) -> String {
        match self {
            Self::TERRAFORM_CLOUD_AUTH_CONFIG => String::from("terraform_cloud_auth_config"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TerraformCloudAuthPersonaMappingType {
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

impl<'de> Deserialize<'de> for TerraformCloudAuthPersonaMappingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"terraform_cloud_auth_config" => Self::TERRAFORM_CLOUD_AUTH_CONFIG,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

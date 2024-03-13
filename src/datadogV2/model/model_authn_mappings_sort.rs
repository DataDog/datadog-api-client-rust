// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthNMappingsSort {
    CREATED_AT_ASCENDING,
    CREATED_AT_DESCENDING,
    ROLE_ID_ASCENDING,
    ROLE_ID_DESCENDING,
    SAML_ASSERTION_ATTRIBUTE_ID_ASCENDING,
    SAML_ASSERTION_ATTRIBUTE_ID_DESCENDING,
    ROLE_NAME_ASCENDING,
    ROLE_NAME_DESCENDING,
    SAML_ASSERTION_ATTRIBUTE_KEY_ASCENDING,
    SAML_ASSERTION_ATTRIBUTE_KEY_DESCENDING,
    SAML_ASSERTION_ATTRIBUTE_VALUE_ASCENDING,
    SAML_ASSERTION_ATTRIBUTE_VALUE_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AuthNMappingsSort {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_AT_ASCENDING => String::from("created_at"),
            Self::CREATED_AT_DESCENDING => String::from("-created_at"),
            Self::ROLE_ID_ASCENDING => String::from("role_id"),
            Self::ROLE_ID_DESCENDING => String::from("-role_id"),
            Self::SAML_ASSERTION_ATTRIBUTE_ID_ASCENDING => {
                String::from("saml_assertion_attribute_id")
            }
            Self::SAML_ASSERTION_ATTRIBUTE_ID_DESCENDING => {
                String::from("-saml_assertion_attribute_id")
            }
            Self::ROLE_NAME_ASCENDING => String::from("role.name"),
            Self::ROLE_NAME_DESCENDING => String::from("-role.name"),
            Self::SAML_ASSERTION_ATTRIBUTE_KEY_ASCENDING => {
                String::from("saml_assertion_attribute.attribute_key")
            }
            Self::SAML_ASSERTION_ATTRIBUTE_KEY_DESCENDING => {
                String::from("-saml_assertion_attribute.attribute_key")
            }
            Self::SAML_ASSERTION_ATTRIBUTE_VALUE_ASCENDING => {
                String::from("saml_assertion_attribute.attribute_value")
            }
            Self::SAML_ASSERTION_ATTRIBUTE_VALUE_DESCENDING => {
                String::from("-saml_assertion_attribute.attribute_value")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AuthNMappingsSort {
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

impl<'de> Deserialize<'de> for AuthNMappingsSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created_at" => Self::CREATED_AT_ASCENDING,
            "-created_at" => Self::CREATED_AT_DESCENDING,
            "role_id" => Self::ROLE_ID_ASCENDING,
            "-role_id" => Self::ROLE_ID_DESCENDING,
            "saml_assertion_attribute_id" => Self::SAML_ASSERTION_ATTRIBUTE_ID_ASCENDING,
            "-saml_assertion_attribute_id" => Self::SAML_ASSERTION_ATTRIBUTE_ID_DESCENDING,
            "role.name" => Self::ROLE_NAME_ASCENDING,
            "-role.name" => Self::ROLE_NAME_DESCENDING,
            "saml_assertion_attribute.attribute_key" => {
                Self::SAML_ASSERTION_ATTRIBUTE_KEY_ASCENDING
            }
            "-saml_assertion_attribute.attribute_key" => {
                Self::SAML_ASSERTION_ATTRIBUTE_KEY_DESCENDING
            }
            "saml_assertion_attribute.attribute_value" => {
                Self::SAML_ASSERTION_ATTRIBUTE_VALUE_ASCENDING
            }
            "-saml_assertion_attribute.attribute_value" => {
                Self::SAML_ASSERTION_ATTRIBUTE_VALUE_DESCENDING
            }
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}

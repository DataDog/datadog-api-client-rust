// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthNMappingsSort {
    #[serde(rename = "created_at")]
    CREATED_AT_ASCENDING,
    #[serde(rename = "-created_at")]
    CREATED_AT_DESCENDING,
    #[serde(rename = "role_id")]
    ROLE_ID_ASCENDING,
    #[serde(rename = "-role_id")]
    ROLE_ID_DESCENDING,
    #[serde(rename = "saml_assertion_attribute_id")]
    SAML_ASSERTION_ATTRIBUTE_ID_ASCENDING,
    #[serde(rename = "-saml_assertion_attribute_id")]
    SAML_ASSERTION_ATTRIBUTE_ID_DESCENDING,
    #[serde(rename = "role.name")]
    ROLE_NAME_ASCENDING,
    #[serde(rename = "-role.name")]
    ROLE_NAME_DESCENDING,
    #[serde(rename = "saml_assertion_attribute.attribute_key")]
    SAML_ASSERTION_ATTRIBUTE_KEY_ASCENDING,
    #[serde(rename = "-saml_assertion_attribute.attribute_key")]
    SAML_ASSERTION_ATTRIBUTE_KEY_DESCENDING,
    #[serde(rename = "saml_assertion_attribute.attribute_value")]
    SAML_ASSERTION_ATTRIBUTE_VALUE_ASCENDING,
    #[serde(rename = "-saml_assertion_attribute.attribute_value")]
    SAML_ASSERTION_ATTRIBUTE_VALUE_DESCENDING,
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
        }
    }
}

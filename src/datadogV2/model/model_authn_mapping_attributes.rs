// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of AuthN Mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingAttributes {
    /// Key portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_key")]
    pub attribute_key: Option<String>,
    /// Value portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_value")]
    pub attribute_value: Option<String>,
    /// Creation time of the AuthN Mapping.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of last AuthN Mapping modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// The ID of the SAML assertion attribute.
    #[serde(rename = "saml_assertion_attribute_id")]
    pub saml_assertion_attribute_id: Option<String>,
}

impl AuthNMappingAttributes {
    pub fn new() -> AuthNMappingAttributes {
        AuthNMappingAttributes {
            attribute_key: None,
            attribute_value: None,
            created_at: None,
            modified_at: None,
            saml_assertion_attribute_id: None,
        }
    }

    pub fn attribute_key(mut self, value: String) -> Self {
        self.attribute_key = Some(value);
        self
    }

    pub fn attribute_value(mut self, value: String) -> Self {
        self.attribute_value = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn saml_assertion_attribute_id(mut self, value: String) -> Self {
        self.saml_assertion_attribute_id = Some(value);
        self
    }
}

impl Default for AuthNMappingAttributes {
    fn default() -> Self {
        Self::new()
    }
}

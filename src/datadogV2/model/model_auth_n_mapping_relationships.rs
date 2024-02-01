// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// All relationships associated with AuthN Mapping.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingRelationships {
    /// Relationship to role.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV2::model::RelationshipToRole>,
    /// AuthN Mapping relationship to SAML Assertion Attribute.
    #[serde(rename = "saml_assertion_attribute")]
    pub saml_assertion_attribute:
        Option<crate::datadogV2::model::RelationshipToSAMLAssertionAttribute>,
}

impl AuthNMappingRelationships {
    pub fn new() -> AuthNMappingRelationships {
        AuthNMappingRelationships {
            role: None,
            saml_assertion_attribute: None,
        }
    }

    pub fn role(&mut self, value: crate::datadogV2::model::RelationshipToRole) -> &mut Self {
        self.role = Some(value);
        self
    }

    pub fn saml_assertion_attribute(
        &mut self,
        value: crate::datadogV2::model::RelationshipToSAMLAssertionAttribute,
    ) -> &mut Self {
        self.saml_assertion_attribute = Some(value);
        self
    }
}

impl Default for AuthNMappingRelationships {
    fn default() -> Self {
        Self::new()
    }
}

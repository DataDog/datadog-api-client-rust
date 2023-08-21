// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingRelationships {
    /// Relationship to role.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: RelationshipToRole,
    /// AuthN Mapping relationship to SAML Assertion Attribute.
    #[serde(rename = "saml_assertion_attribute")]
    pub saml_assertion_attribute: RelationshipToSAMLAssertionAttribute,
}


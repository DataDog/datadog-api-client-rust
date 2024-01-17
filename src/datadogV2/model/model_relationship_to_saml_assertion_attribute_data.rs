// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data of AuthN Mapping relationship to SAML Assertion Attribute.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToSAMLAssertionAttributeData {
    /// The ID of the SAML assertion attribute.
    #[serde(rename = "id")]
    pub id: String,
    /// SAML assertion attributes resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SAMLAssertionAttributesType,
}

impl RelationshipToSAMLAssertionAttributeData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::SAMLAssertionAttributesType,
    ) -> RelationshipToSAMLAssertionAttributeData {
        RelationshipToSAMLAssertionAttributeData { id, type_ }
    }
}

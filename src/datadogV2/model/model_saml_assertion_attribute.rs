// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// SAML assertion attribute.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SAMLAssertionAttribute {
    /// Key/Value pair of attributes used in SAML assertion attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SAMLAssertionAttributeAttributes>,
    /// The ID of the SAML assertion attribute.
    #[serde(rename = "id")]
    pub id: String,
    /// SAML assertion attributes resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SAMLAssertionAttributesType,
}

impl SAMLAssertionAttribute {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::SAMLAssertionAttributesType,
    ) -> SAMLAssertionAttribute {
        SAMLAssertionAttribute {
            attributes: None,
            id,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::SAMLAssertionAttributeAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Key/Value pair of attributes used in SAML assertion attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SAMLAssertionAttributeAttributes {
    /// Key portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_key")]
    pub attribute_key: Option<String>,
    /// Value portion of a key/value pair of the attribute sent from the Identity Provider.
    #[serde(rename = "attribute_value")]
    pub attribute_value: Option<String>,
}

impl SAMLAssertionAttributeAttributes {
    pub fn new() -> SAMLAssertionAttributeAttributes {
        SAMLAssertionAttributeAttributes {
            attribute_key: None,
            attribute_value: None,
        }
    }
}
impl Default for SAMLAssertionAttributeAttributes {
    fn default() -> Self {
        Self::new()
    }
}

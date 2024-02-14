// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AuthN Mapping relationship to SAML Assertion Attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToSAMLAssertionAttribute {
    /// Data of AuthN Mapping relationship to SAML Assertion Attribute.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::RelationshipToSAMLAssertionAttributeData,
}

impl RelationshipToSAMLAssertionAttribute {
    pub fn new(
        data: crate::datadogV2::model::RelationshipToSAMLAssertionAttributeData,
    ) -> RelationshipToSAMLAssertionAttribute {
        RelationshipToSAMLAssertionAttribute { data }
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The AuthN Mapping object returned by API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMapping {
    /// Attributes of AuthN Mapping.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::AuthNMappingAttributes>>,
    /// ID of the AuthN Mapping.
    #[serde(rename = "id")]
    pub id: String,
    /// All relationships associated with AuthN Mapping.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::AuthNMappingRelationships>>,
    /// AuthN Mappings resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AuthNMappingsType,
}

impl AuthNMapping {
    pub fn new(id: String, type_: crate::datadogV2::model::AuthNMappingsType) -> AuthNMapping {
        AuthNMapping {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }
}

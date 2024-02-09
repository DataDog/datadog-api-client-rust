// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data for updating an AuthN Mapping.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingUpdateData {
    /// Key/Value pair of attributes used for update request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::AuthNMappingUpdateAttributes>,
    /// ID of the AuthN Mapping.
    #[serde(rename = "id")]
    pub id: String,
    /// Relationship of AuthN Mapping update object to Role.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::AuthNMappingUpdateRelationships>,
    /// AuthN Mappings resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AuthNMappingsType,
}

impl AuthNMappingUpdateData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::AuthNMappingsType,
    ) -> AuthNMappingUpdateData {
        AuthNMappingUpdateData {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingUpdateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingUpdateRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}

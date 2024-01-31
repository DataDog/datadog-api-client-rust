// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data for creating an AuthN Mapping.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingCreateData {
    /// Key/Value pair of attributes used for create request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::AuthNMappingCreateAttributes>,
    /// Relationship of AuthN Mapping create object to Role.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::AuthNMappingCreateRelationships>,
    /// AuthN Mappings resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AuthNMappingsType,
}

impl AuthNMappingCreateData {
    pub fn new(type_: crate::datadogV2::model::AuthNMappingsType) -> AuthNMappingCreateData {
        AuthNMappingCreateData {
            attributes: None,
            relationships: None,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingCreateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_relationships(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingCreateRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Role object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCreateResponseData {
    /// Attributes of the created role.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RoleCreateAttributes>,
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the role object returned by the API.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::RoleResponseRelationships>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RolesType,
}

impl RoleCreateResponseData {
    pub fn new(type_: crate::datadogV2::model::RolesType) -> RoleCreateResponseData {
        RoleCreateResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::RoleCreateAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::RoleResponseRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}

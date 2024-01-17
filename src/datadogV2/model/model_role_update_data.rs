// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the update of a role.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleUpdateData {
    /// Attributes of the role.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::RoleUpdateAttributes>,
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: String,
    /// Relationships of the role object.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::RoleRelationships>>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RolesType,
}

impl RoleUpdateData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::RoleUpdateAttributes>,
        id: String,
        type_: crate::datadogV2::model::RolesType,
    ) -> RoleUpdateData {
        RoleUpdateData {
            attributes,
            id,
            relationships: None,
            type_,
        }
    }
}

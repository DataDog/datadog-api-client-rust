// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Role object returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleUpdateResponseData {
    /// Attributes of the role.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::RoleUpdateAttributes>>,
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the role object returned by the API.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::RoleResponseRelationships>>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RolesType,
}

impl RoleUpdateResponseData {
    pub fn new(type_: crate::datadogV2::model::RolesType) -> RoleUpdateResponseData {
        RoleUpdateResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
        }
    }
}

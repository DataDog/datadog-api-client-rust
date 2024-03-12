// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the creation of a role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCreateData {
    /// Attributes of the created role.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::RoleCreateAttributes,
    /// Relationships of the role object.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::RoleRelationships>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RolesType>,
}

impl RoleCreateData {
    pub fn new(attributes: crate::datadogV2::model::RoleCreateAttributes) -> RoleCreateData {
        RoleCreateData {
            attributes,
            relationships: None,
            type_: None,
        }
    }

    pub fn relationships(mut self, value: crate::datadogV2::model::RoleRelationships) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::RolesType) -> Self {
        self.type_ = Some(value);
        self
    }
}

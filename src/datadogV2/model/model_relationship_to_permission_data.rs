// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to permission object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToPermissionData {
    /// ID of the permission.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Permissions resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::PermissionsType>,
}

impl RelationshipToPermissionData {
    pub fn new() -> RelationshipToPermissionData {
        RelationshipToPermissionData {
            id: None,
            type_: None,
        }
    }
}

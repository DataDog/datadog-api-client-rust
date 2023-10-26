// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    /// Attributes of a permission.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::PermissionAttributes>>,
    /// ID of the permission.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Permissions resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::PermissionsType,
}

impl Permission {
    /// Permission object.
    pub fn new(type_: crate::datadogV2::model::PermissionsType) -> Permission {
        Permission {
            attributes: None,
            id: None,
            type_: type_,
        }
    }
}

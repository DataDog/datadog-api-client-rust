// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Permission object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    /// Attributes of a permission.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::PermissionAttributes>>,
    /// ID of the permission.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Permissions resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::PermissionsType,
}

impl Permission {
    pub fn new(type_: crate::datadogV2::model::PermissionsType) -> Permission {
        Permission {
            attributes: None,
            id: None,
            type_,
        }
    }
}

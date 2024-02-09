// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data for the clone role request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleClone {
    /// Attributes required to create a new role by cloning an existing one.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::RoleCloneAttributes,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RolesType,
}

impl RoleClone {
    pub fn new(
        attributes: crate::datadogV2::model::RoleCloneAttributes,
        type_: crate::datadogV2::model::RolesType,
    ) -> RoleClone {
        RoleClone { attributes, type_ }
    }
}

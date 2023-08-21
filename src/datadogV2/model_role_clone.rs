// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleClone {
    /// Attributes required to create a new role by cloning an existing one.
    #[serde(rename = "attributes")]
    pub attributes: RoleCloneAttributes,
    /// Roles type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: RolesType,
}


// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A user's permissions for a given team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamPermission {
    /// User team permission attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::UserTeamPermissionAttributes>>,
    /// The user team permission's identifier
    #[serde(rename = "id")]
    pub id: String,
    /// User team permission type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamPermissionType,
}

impl UserTeamPermission {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::UserTeamPermissionType,
    ) -> UserTeamPermission {
        UserTeamPermission {
            attributes: None,
            id,
            type_,
        }
    }
}

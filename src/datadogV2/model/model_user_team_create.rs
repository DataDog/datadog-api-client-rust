// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A user's relationship with a team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamCreate {
    /// Team membership attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::UserTeamAttributes>>,
    /// Relationship between membership and a user
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::UserTeamRelationships>>,
    /// Team membership type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamType,
}

impl UserTeamCreate {
    pub fn new(type_: crate::datadogV2::model::UserTeamType) -> UserTeamCreate {
        UserTeamCreate {
            attributes: None,
            relationships: None,
            type_,
        }
    }
}

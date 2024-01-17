// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team memberships response
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamsResponse {
    /// Team memberships response data
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::UserTeam>>,
    /// Teams response links.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::TeamsResponseLinks>>,
    /// Teams response metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::TeamsResponseMeta>>,
}

impl UserTeamsResponse {
    pub fn new() -> UserTeamsResponse {
        UserTeamsResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}

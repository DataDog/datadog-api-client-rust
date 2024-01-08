// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team membership response
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamResponse {
    /// A user's relationship with a team
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::UserTeam>>,
}

impl UserTeamResponse {
    pub fn new() -> UserTeamResponse {
        UserTeamResponse { data: None }
    }
}
impl Default for UserTeamResponse {
    fn default() -> Self {
        Self::new()
    }
}

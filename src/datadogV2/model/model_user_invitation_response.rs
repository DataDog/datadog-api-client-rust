// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// User invitation as returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationResponse {
    /// Object of a user invitation returned by the API.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::UserInvitationResponseData>>,
}

impl UserInvitationResponse {
    pub fn new() -> UserInvitationResponse {
        UserInvitationResponse { data: None }
    }
}
impl Default for UserInvitationResponse {
    fn default() -> Self {
        Self::new()
    }
}

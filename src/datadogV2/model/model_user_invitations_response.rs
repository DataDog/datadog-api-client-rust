// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// User invitations as returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationsResponse {
    /// Array of user invitations.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::UserInvitationResponseData>>,
}

impl UserInvitationsResponse {
    pub fn new() -> UserInvitationsResponse {
        UserInvitationsResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::UserInvitationResponseData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for UserInvitationsResponse {
    fn default() -> Self {
        Self::new()
    }
}

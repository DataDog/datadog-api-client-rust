// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to invite users to join the organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationsRequest {
    /// List of user invitations.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::UserInvitationData>,
}

impl UserInvitationsRequest {
    pub fn new(data: Vec<crate::datadogV2::model::UserInvitationData>) -> UserInvitationsRequest {
        UserInvitationsRequest { data }
    }
}
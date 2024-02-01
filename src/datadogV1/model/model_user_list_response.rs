// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Array of Datadog users for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListResponse {
    /// Array of users.
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV1::model::User>>,
}

impl UserListResponse {
    pub fn new() -> UserListResponse {
        UserListResponse { users: None }
    }

    pub fn users(&mut self, value: Vec<crate::datadogV1::model::User>) -> &mut Self {
        self.users = Some(value);
        self
    }
}

impl Default for UserListResponse {
    fn default() -> Self {
        Self::new()
    }
}

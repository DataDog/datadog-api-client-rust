// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A Datadog User.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    /// Create, edit, and disable users.
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV1::model::User>,
}

impl UserResponse {
    pub fn new() -> UserResponse {
        UserResponse { user: None }
    }

    pub fn user(&mut self, value: crate::datadogV1::model::User) -> &mut Self {
        self.user = Some(value);
        self
    }
}

impl Default for UserResponse {
    fn default() -> Self {
        Self::new()
    }
}

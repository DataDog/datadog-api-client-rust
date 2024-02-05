// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Array of user disabled for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDisableResponse {
    /// Information pertaining to a user disabled for a given organization.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl UserDisableResponse {
    pub fn new() -> UserDisableResponse {
        UserDisableResponse { message: None }
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }
}

impl Default for UserDisableResponse {
    fn default() -> Self {
        Self::new()
    }
}

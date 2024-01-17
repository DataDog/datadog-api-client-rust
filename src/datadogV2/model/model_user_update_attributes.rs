// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the edited user.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateAttributes {
    /// If the user is enabled or disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl UserUpdateAttributes {
    pub fn new() -> UserUpdateAttributes {
        UserUpdateAttributes {
            disabled: None,
            email: None,
            name: None,
        }
    }
}
impl Default for UserUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

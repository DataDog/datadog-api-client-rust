// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the created user.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateAttributes {
    /// The email of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The title of the user.
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl UserCreateAttributes {
    pub fn new(email: String) -> UserCreateAttributes {
        UserCreateAttributes {
            email,
            name: None,
            title: None,
        }
    }
}

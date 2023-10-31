// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create, edit, and disable users.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
    #[serde(rename = "access_role", default, with = "::serde_with::rust::double_option")]
    pub access_role: Option<Option<crate::datadogV1::model::AccessRole>>,
    /// The new disabled status of the user.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// The new email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The user handle, must be a valid email.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether or not the user logged in Datadog at least once.
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
}

impl User {
    pub fn new() -> User {
        User {
            access_role: None,
            disabled: None,
            email: None,
            handle: None,
            icon: None,
            name: None,
            verified: None,
        }
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
    #[serde(rename = "access_role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub access_role: NullableAccessRole,
    /// The new disabled status of the user.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: bool,
    /// The new email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// The user handle, must be a valid email.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: String,
    /// The name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Whether or not the user logged in Datadog at least once.
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: bool,
}


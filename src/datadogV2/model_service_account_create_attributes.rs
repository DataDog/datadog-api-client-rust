// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCreateAttributes {
    /// The email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// The name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Whether the user is a service account. Must be true.
    #[serde(rename = "service_account", skip_serializing_if = "Option::is_none")]
    pub service_account: bool,
    /// The title of the user.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}


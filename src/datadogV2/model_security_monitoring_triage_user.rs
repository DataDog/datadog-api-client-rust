// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringTriageUser {
    /// The handle for this user account.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: String,
    /// Numerical ID assigned by Datadog to this user account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// The name for this user account.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// UUID assigned by Datadog to this user account.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: String,
}


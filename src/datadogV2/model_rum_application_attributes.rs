// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationAttributes {
    /// ID of the RUM application.
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: String,
    /// Client token of the RUM application.
    #[serde(rename = "client_token", skip_serializing_if = "Option::is_none")]
    pub client_token: String,
    /// Timestamp in ms of the creation date.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: i64,
    /// Handle of the creator user.
    #[serde(rename = "created_by_handle", skip_serializing_if = "Option::is_none")]
    pub created_by_handle: String,
    /// Hash of the RUM application. Optional.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: String,
    /// Indicates if the RUM application is active.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: bool,
    /// Name of the RUM application.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Org ID of the RUM application.
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: i32,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
    /// Timestamp in ms of the last update date.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: i64,
    /// Handle of the updater user.
    #[serde(rename = "updated_by_handle", skip_serializing_if = "Option::is_none")]
    pub updated_by_handle: String,
}


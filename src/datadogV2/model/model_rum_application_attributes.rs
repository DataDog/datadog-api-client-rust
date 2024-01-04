// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationAttributes {
    /// ID of the RUM application.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Client token of the RUM application.
    #[serde(rename = "client_token")]
    pub client_token: String,
    /// Timestamp in ms of the creation date.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Handle of the creator user.
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: String,
    /// Hash of the RUM application. Optional.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// Indicates if the RUM application is active.
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: String,
    /// Org ID of the RUM application.
    #[serde(rename = "org_id")]
    pub org_id: i32,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Timestamp in ms of the last update date.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    /// Handle of the updater user.
    #[serde(rename = "updated_by_handle")]
    pub updated_by_handle: String,
}

impl RUMApplicationAttributes {
    pub fn new(
        application_id: String,
        client_token: String,
        created_at: i64,
        created_by_handle: String,
        name: String,
        org_id: i32,
        type_: String,
        updated_at: i64,
        updated_by_handle: String,
    ) -> RUMApplicationAttributes {
        RUMApplicationAttributes {
            application_id,
            client_token,
            created_at,
            created_by_handle,
            hash: None,
            is_active: None,
            name,
            org_id,
            type_,
            updated_at,
            updated_by_handle,
        }
    }
}
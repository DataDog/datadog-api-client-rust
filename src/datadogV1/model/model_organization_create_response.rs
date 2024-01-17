// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object for an organization creation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreateResponse {
    /// Datadog API key.
    #[serde(rename = "api_key")]
    pub api_key: Option<Box<crate::datadogV1::model::ApiKey>>,
    /// An application key with its associated metadata.
    #[serde(rename = "application_key")]
    pub application_key: Option<Box<crate::datadogV1::model::ApplicationKey>>,
    /// Create, edit, and manage organizations.
    #[serde(rename = "org")]
    pub org: Option<Box<crate::datadogV1::model::Organization>>,
    /// Create, edit, and disable users.
    #[serde(rename = "user")]
    pub user: Option<Box<crate::datadogV1::model::User>>,
}

impl OrganizationCreateResponse {
    pub fn new() -> OrganizationCreateResponse {
        OrganizationCreateResponse {
            api_key: None,
            application_key: None,
            org: None,
            user: None,
        }
    }
}
impl Default for OrganizationCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}

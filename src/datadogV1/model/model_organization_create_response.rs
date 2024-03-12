// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object for an organization creation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreateResponse {
    /// Datadog API key.
    #[serde(rename = "api_key")]
    pub api_key: Option<crate::datadogV1::model::ApiKey>,
    /// An application key with its associated metadata.
    #[serde(rename = "application_key")]
    pub application_key: Option<crate::datadogV1::model::ApplicationKey>,
    /// Create, edit, and manage organizations.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV1::model::Organization>,
    /// Create, edit, and disable users.
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV1::model::User>,
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

    pub fn api_key(mut self, value: crate::datadogV1::model::ApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn application_key(mut self, value: crate::datadogV1::model::ApplicationKey) -> Self {
        self.application_key = Some(value);
        self
    }

    pub fn org(mut self, value: crate::datadogV1::model::Organization) -> Self {
        self.org = Some(value);
        self
    }

    pub fn user(mut self, value: crate::datadogV1::model::User) -> Self {
        self.user = Some(value);
        self
    }
}

impl Default for OrganizationCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}

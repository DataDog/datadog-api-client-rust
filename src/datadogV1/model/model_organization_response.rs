// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResponse {
    /// Create, edit, and manage organizations.
    #[serde(rename = "org")]
    pub org: Option<crate::datadogV1::model::Organization>,
}

impl OrganizationResponse {
    pub fn new() -> OrganizationResponse {
        OrganizationResponse { org: None }
    }

    pub fn org(mut self, value: crate::datadogV1::model::Organization) -> Self {
        self.org = Some(value);
        self
    }
}

impl Default for OrganizationResponse {
    fn default() -> Self {
        Self::new()
    }
}

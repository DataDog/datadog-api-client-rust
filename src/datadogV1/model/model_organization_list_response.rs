// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with the list of organizations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationListResponse {
    /// Array of organization objects.
    #[serde(rename = "orgs")]
    pub orgs: Option<Vec<crate::datadogV1::model::Organization>>,
}

impl OrganizationListResponse {
    pub fn new() -> OrganizationListResponse {
        OrganizationListResponse { orgs: None }
    }

    pub fn orgs(&mut self, value: Vec<crate::datadogV1::model::Organization>) -> &mut Self {
        self.orgs = Some(value);
        self
    }
}

impl Default for OrganizationListResponse {
    fn default() -> Self {
        Self::new()
    }
}

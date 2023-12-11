// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResponse {
    /// Create, edit, and manage organizations.
    #[serde(rename = "org")]
    pub org: Option<Box<crate::datadogV1::model::Organization>>,
}

impl OrganizationResponse {
    pub fn new() -> OrganizationResponse {
        OrganizationResponse { org: None }
    }
}

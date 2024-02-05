// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response containing the Cloud Security Management Pro usage for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCloudSecurityPostureManagementResponse {
    /// Get hourly usage for Cloud Security Management Pro.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageCloudSecurityPostureManagementHour>>,
}

impl UsageCloudSecurityPostureManagementResponse {
    pub fn new() -> UsageCloudSecurityPostureManagementResponse {
        UsageCloudSecurityPostureManagementResponse { usage: None }
    }

    pub fn usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageCloudSecurityPostureManagementHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageCloudSecurityPostureManagementResponse {
    fn default() -> Self {
        Self::new()
    }
}

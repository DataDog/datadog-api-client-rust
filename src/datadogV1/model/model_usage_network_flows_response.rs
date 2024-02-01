// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of netflow events indexed for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageNetworkFlowsResponse {
    /// Get hourly usage for Network Flows.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageNetworkFlowsHour>>,
}

impl UsageNetworkFlowsResponse {
    pub fn new() -> UsageNetworkFlowsResponse {
        UsageNetworkFlowsResponse { usage: None }
    }

    pub fn usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageNetworkFlowsHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageNetworkFlowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of profiled hosts for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageProfilingResponse {
    /// Get hourly usage for profiled hosts.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageProfilingHour>>,
}

impl UsageProfilingResponse {
    pub fn new() -> UsageProfilingResponse {
        UsageProfilingResponse { usage: None }
    }

    pub fn usage(&mut self, value: Vec<crate::datadogV1::model::UsageProfilingHour>) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageProfilingResponse {
    fn default() -> Self {
        Self::new()
    }
}

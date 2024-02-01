// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the Sensitive Data Scanner usage for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSDSResponse {
    /// Get hourly usage for Sensitive Data Scanner.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageSDSHour>>,
}

impl UsageSDSResponse {
    pub fn new() -> UsageSDSResponse {
        UsageSDSResponse { usage: None }
    }

    pub fn usage(&mut self, value: Vec<crate::datadogV1::model::UsageSDSHour>) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageSDSResponse {
    fn default() -> Self {
        Self::new()
    }
}

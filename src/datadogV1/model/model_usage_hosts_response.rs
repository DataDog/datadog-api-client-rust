// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Host usage response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageHostsResponse {
    /// An array of objects related to host usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageHostHour>>,
}

impl UsageHostsResponse {
    pub fn new() -> UsageHostsResponse {
        UsageHostsResponse { usage: None }
    }

    pub fn usage(&mut self, value: Vec<crate::datadogV1::model::UsageHostHour>) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageHostsResponse {
    fn default() -> Self {
        Self::new()
    }
}

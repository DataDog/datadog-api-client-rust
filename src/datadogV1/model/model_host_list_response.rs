// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with Host information from Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostListResponse {
    /// Array of hosts.
    #[serde(rename = "host_list")]
    pub host_list: Option<Vec<crate::datadogV1::model::Host>>,
    /// Number of host matching the query.
    #[serde(rename = "total_matching")]
    pub total_matching: Option<i64>,
    /// Number of host returned.
    #[serde(rename = "total_returned")]
    pub total_returned: Option<i64>,
}

impl HostListResponse {
    pub fn new() -> HostListResponse {
        HostListResponse {
            host_list: None,
            total_matching: None,
            total_returned: None,
        }
    }

    pub fn host_list(&mut self, value: Vec<crate::datadogV1::model::Host>) -> &mut Self {
        self.host_list = Some(value);
        self
    }

    pub fn total_matching(&mut self, value: i64) -> &mut Self {
        self.total_matching = Some(value);
        self
    }

    pub fn total_returned(&mut self, value: i64) -> &mut Self {
        self.total_returned = Some(value);
        self
    }
}

impl Default for HostListResponse {
    fn default() -> Self {
        Self::new()
    }
}

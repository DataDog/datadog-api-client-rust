// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The paging attributes for listing security signals.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalListRequestPage {
    /// A list of results using the cursor provided in the previous query.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// The maximum number of security signals in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
}

impl SecurityMonitoringSignalListRequestPage {
    pub fn new() -> SecurityMonitoringSignalListRequestPage {
        SecurityMonitoringSignalListRequestPage {
            cursor: None,
            limit: None,
        }
    }

    pub fn with_cursor(&mut self, value: String) -> &mut Self {
        self.cursor = Some(value);
        self
    }

    pub fn with_limit(&mut self, value: i32) -> &mut Self {
        self.limit = Some(value);
        self
    }
}
impl Default for SecurityMonitoringSignalListRequestPage {
    fn default() -> Self {
        Self::new()
    }
}

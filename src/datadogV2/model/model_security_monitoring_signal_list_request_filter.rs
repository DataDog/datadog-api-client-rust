// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Search filters for listing security signals.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalListRequestFilter {
    /// The minimum timestamp for requested security signals.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// Search query for listing security signals.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum timestamp for requested security signals.
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl SecurityMonitoringSignalListRequestFilter {
    pub fn new() -> SecurityMonitoringSignalListRequestFilter {
        SecurityMonitoringSignalListRequestFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}
impl Default for SecurityMonitoringSignalListRequestFilter {
    fn default() -> Self {
        Self::new()
    }
}

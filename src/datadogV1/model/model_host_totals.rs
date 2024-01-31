// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Total number of host currently monitored by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostTotals {
    /// Total number of active host (UP and ???) reporting to Datadog.
    #[serde(rename = "total_active")]
    pub total_active: Option<i64>,
    /// Number of host that are UP and reporting to Datadog.
    #[serde(rename = "total_up")]
    pub total_up: Option<i64>,
}

impl HostTotals {
    pub fn new() -> HostTotals {
        HostTotals {
            total_active: None,
            total_up: None,
        }
    }

    pub fn with_total_active(&mut self, value: i64) -> &mut Self {
        self.total_active = Some(value);
        self
    }

    pub fn with_total_up(&mut self, value: i64) -> &mut Self {
        self.total_up = Some(value);
        self
    }
}
impl Default for HostTotals {
    fn default() -> Self {
        Self::new()
    }
}

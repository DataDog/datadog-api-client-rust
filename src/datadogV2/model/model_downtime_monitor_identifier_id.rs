// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object of the monitor identifier.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeMonitorIdentifierId {
    /// ID of the monitor to prevent notifications.
    #[serde(rename = "monitor_id")]
    pub monitor_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl DowntimeMonitorIdentifierId {
    pub fn new(monitor_id: i64) -> DowntimeMonitorIdentifierId {
        DowntimeMonitorIdentifierId {
            monitor_id,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}

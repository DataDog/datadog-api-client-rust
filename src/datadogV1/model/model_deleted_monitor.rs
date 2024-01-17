// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response from the delete monitor call.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedMonitor {
    /// ID of the deleted monitor.
    #[serde(rename = "deleted_monitor_id")]
    pub deleted_monitor_id: Option<i64>,
}

impl DeletedMonitor {
    pub fn new() -> DeletedMonitor {
        DeletedMonitor {
            deleted_monitor_id: None,
        }
    }
}

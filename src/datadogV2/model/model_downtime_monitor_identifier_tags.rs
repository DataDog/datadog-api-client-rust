// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object of the monitor tags.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeMonitorIdentifierTags {
    /// A list of monitor tags. For example, tags that are applied directly to monitors,
    /// not tags that are used in monitor queries (which are filtered by the scope parameter), to which the downtime applies.
    /// The resulting downtime applies to monitors that match **all** provided monitor tags. Setting `monitor_tags`
    /// to `[*]` configures the downtime to mute all monitors for the given scope.
    #[serde(rename = "monitor_tags")]
    pub monitor_tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl DowntimeMonitorIdentifierTags {
    pub fn new(monitor_tags: Vec<String>) -> DowntimeMonitorIdentifierTags {
        DowntimeMonitorIdentifierTags {
            monitor_tags,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}

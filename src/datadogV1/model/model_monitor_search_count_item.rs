// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A facet item.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchCountItem {
    /// The number of found monitors with the listed value.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// The facet value.
    #[serde(rename = "name")]
    pub name: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl MonitorSearchCountItem {
    pub fn new() -> MonitorSearchCountItem {
        MonitorSearchCountItem {
            count: None,
            name: None,
        }
    }
}

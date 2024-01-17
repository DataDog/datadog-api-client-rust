// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Information about the monitor identified by the downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeMonitorIncludedItem {
    /// Attributes of the monitor identified by the downtime.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::DowntimeMonitorIncludedAttributes>>,
    /// ID of the monitor identified by the downtime.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Monitor resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DowntimeIncludedMonitorType>,
}

impl DowntimeMonitorIncludedItem {
    pub fn new() -> DowntimeMonitorIncludedItem {
        DowntimeMonitorIncludedItem {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}

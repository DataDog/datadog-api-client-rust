// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data for the monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRelationshipsMonitorData {
    /// Monitor ID of the downtime.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Monitor resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DowntimeIncludedMonitorType>,
}

impl DowntimeRelationshipsMonitorData {
    pub fn new() -> DowntimeRelationshipsMonitorData {
        DowntimeRelationshipsMonitorData {
            id: None,
            type_: None,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::DowntimeIncludedMonitorType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for DowntimeRelationshipsMonitorData {
    fn default() -> Self {
        Self::new()
    }
}

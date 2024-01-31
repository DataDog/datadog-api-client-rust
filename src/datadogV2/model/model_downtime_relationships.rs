// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// All relationships associated with downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRelationships {
    /// The user who created the downtime.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::DowntimeRelationshipsCreatedBy>,
    /// The monitor identified by the downtime.
    #[serde(rename = "monitor")]
    pub monitor: Option<crate::datadogV2::model::DowntimeRelationshipsMonitor>,
}

impl DowntimeRelationships {
    pub fn new() -> DowntimeRelationships {
        DowntimeRelationships {
            created_by: None,
            monitor: None,
        }
    }

    pub fn with_created_by(
        &mut self,
        value: crate::datadogV2::model::DowntimeRelationshipsCreatedBy,
    ) -> &mut Self {
        self.created_by = Some(value);
        self
    }

    pub fn with_monitor(
        &mut self,
        value: crate::datadogV2::model::DowntimeRelationshipsMonitor,
    ) -> &mut Self {
        self.monitor = Some(value);
        self
    }
}
impl Default for DowntimeRelationships {
    fn default() -> Self {
        Self::new()
    }
}

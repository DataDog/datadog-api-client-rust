// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of indexed logs for each hour and index for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByIndexHour {
    /// The total number of indexed logs for the queried hour.
    #[serde(rename = "event_count")]
    pub event_count: Option<i64>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The index ID for this usage.
    #[serde(rename = "index_id")]
    pub index_id: Option<String>,
    /// The user specified name for this index ID.
    #[serde(rename = "index_name")]
    pub index_name: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The retention period (in days) for this index ID.
    #[serde(rename = "retention")]
    pub retention: Option<i64>,
}

impl UsageLogsByIndexHour {
    pub fn new() -> UsageLogsByIndexHour {
        UsageLogsByIndexHour {
            event_count: None,
            hour: None,
            index_id: None,
            index_name: None,
            org_name: None,
            public_id: None,
            retention: None,
        }
    }

    pub fn with_event_count(&mut self, value: i64) -> &mut Self {
        self.event_count = Some(value);
        self
    }

    pub fn with_hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn with_index_id(&mut self, value: String) -> &mut Self {
        self.index_id = Some(value);
        self
    }

    pub fn with_index_name(&mut self, value: String) -> &mut Self {
        self.index_name = Some(value);
        self
    }

    pub fn with_org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn with_public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn with_retention(&mut self, value: i64) -> &mut Self {
        self.retention = Some(value);
        self
    }
}
impl Default for UsageLogsByIndexHour {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The number of indexed logs for each hour for a given organization broken down by retention period.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByRetentionHour {
    /// Total logs indexed with this retention period during a given hour.
    #[serde(
        rename = "indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub indexed_events_count: Option<Option<i64>>,
    /// Live logs indexed with this retention period during a given hour.
    #[serde(
        rename = "live_indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub live_indexed_events_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Rehydrated logs indexed with this retention period during a given hour.
    #[serde(
        rename = "rehydrated_indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rehydrated_indexed_events_count: Option<Option<i64>>,
    /// The retention period in days or "custom" for all custom retention usage.
    #[serde(
        rename = "retention",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub retention: Option<Option<String>>,
}

impl UsageLogsByRetentionHour {
    pub fn new() -> UsageLogsByRetentionHour {
        UsageLogsByRetentionHour {
            indexed_events_count: None,
            live_indexed_events_count: None,
            org_name: None,
            public_id: None,
            rehydrated_indexed_events_count: None,
            retention: None,
        }
    }
}

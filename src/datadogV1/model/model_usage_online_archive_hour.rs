// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Online Archive usage in a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageOnlineArchiveHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Total count of online archived events within the hour.
    #[serde(
        rename = "online_archive_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub online_archive_events_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageOnlineArchiveHour {
    pub fn new() -> UsageOnlineArchiveHour {
        UsageOnlineArchiveHour {
            hour: None,
            online_archive_events_count: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn online_archive_events_count(&mut self, value: Option<i64>) -> &mut Self {
        self.online_archive_events_count = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageOnlineArchiveHour {
    fn default() -> Self {
        Self::new()
    }
}

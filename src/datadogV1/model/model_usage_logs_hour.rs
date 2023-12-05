// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Hour usage for logs.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsHour {
    /// Contains the number of billable log bytes ingested.
    #[serde(
        rename = "billable_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub billable_ingested_bytes: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Contains the number of log events indexed.
    #[serde(
        rename = "indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub indexed_events_count: Option<Option<i64>>,
    /// Contains the number of log bytes ingested.
    #[serde(
        rename = "ingested_events_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ingested_events_bytes: Option<Option<i64>>,
    /// Contains the number of logs forwarded bytes (data available as of April 1st 2023)
    #[serde(
        rename = "logs_forwarding_events_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_forwarding_events_bytes: Option<Option<i64>>,
    /// Contains the number of live log events indexed (data available as of December 1, 2020).
    #[serde(
        rename = "logs_live_indexed_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_live_indexed_count: Option<Option<i64>>,
    /// Contains the number of live log bytes ingested (data available as of December 1, 2020).
    #[serde(
        rename = "logs_live_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_live_ingested_bytes: Option<Option<i64>>,
    /// Contains the number of rehydrated log events indexed (data available as of December 1, 2020).
    #[serde(
        rename = "logs_rehydrated_indexed_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_rehydrated_indexed_count: Option<Option<i64>>,
    /// Contains the number of rehydrated log bytes ingested (data available as of December 1, 2020).
    #[serde(
        rename = "logs_rehydrated_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_rehydrated_ingested_bytes: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageLogsHour {
    pub fn new() -> UsageLogsHour {
        UsageLogsHour {
            billable_ingested_bytes: None,
            hour: None,
            indexed_events_count: None,
            ingested_events_bytes: None,
            logs_forwarding_events_bytes: None,
            logs_live_indexed_count: None,
            logs_live_ingested_bytes: None,
            logs_rehydrated_indexed_count: None,
            logs_rehydrated_ingested_bytes: None,
            org_name: None,
            public_id: None,
        }
    }
}
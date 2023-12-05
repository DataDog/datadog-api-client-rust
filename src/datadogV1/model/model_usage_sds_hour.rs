// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Sensitive Data Scanner usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageSDSHour {
    /// The total number of bytes scanned of APM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "apm_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub apm_scanned_bytes: Option<Option<i64>>,
    /// The total number of bytes scanned of Events usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "events_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub events_scanned_bytes: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The total number of bytes scanned of logs usage by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "logs_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_scanned_bytes: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The total number of bytes scanned of RUM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "rum_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rum_scanned_bytes: Option<Option<i64>>,
    /// The total number of bytes scanned across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "total_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_scanned_bytes: Option<Option<i64>>,
}

impl UsageSDSHour {
    pub fn new() -> UsageSDSHour {
        UsageSDSHour {
            apm_scanned_bytes: None,
            events_scanned_bytes: None,
            hour: None,
            logs_scanned_bytes: None,
            org_name: None,
            public_id: None,
            rum_scanned_bytes: None,
            total_scanned_bytes: None,
        }
    }
}
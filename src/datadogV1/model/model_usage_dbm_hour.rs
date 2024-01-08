// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Database Monitoring usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDBMHour {
    /// The total number of Database Monitoring host hours from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "dbm_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dbm_host_count: Option<Option<i64>>,
    /// The total number of normalized Database Monitoring queries from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "dbm_queries_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dbm_queries_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageDBMHour {
    pub fn new() -> UsageDBMHour {
        UsageDBMHour {
            dbm_host_count: None,
            dbm_queries_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}
impl Default for UsageDBMHour {
    fn default() -> Self {
        Self::new()
    }
}

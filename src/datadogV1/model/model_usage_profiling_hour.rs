// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The number of profiled hosts for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageProfilingHour {
    /// Contains the total number of profiled Azure app services reporting during a given hour.
    #[serde(rename = "aas_count", default, with = "::serde_with::rust::double_option")]
    pub aas_count: Option<Option<i64>>,
    /// Get average number of container agents for that hour.
    #[serde(
        rename = "avg_container_agent_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub avg_container_agent_count: Option<Option<i64>>,
    /// Contains the total number of profiled hosts reporting during a given hour.
    #[serde(rename = "host_count", default, with = "::serde_with::rust::double_option")]
    pub host_count: Option<Option<i64>>,
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

impl UsageProfilingHour {
    pub fn new() -> UsageProfilingHour {
        UsageProfilingHour {
            aas_count: None,
            avg_container_agent_count: None,
            host_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}

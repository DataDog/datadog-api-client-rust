// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Cloud Workload Security usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageCWSHour {
    /// The total number of Cloud Workload Security container hours from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "cws_container_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cws_container_count: Option<Option<i64>>,
    /// The total number of Cloud Workload Security host hours from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "cws_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cws_host_count: Option<Option<i64>>,
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

impl UsageCWSHour {
    pub fn new() -> UsageCWSHour {
        UsageCWSHour {
            cws_container_count: None,
            cws_host_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}
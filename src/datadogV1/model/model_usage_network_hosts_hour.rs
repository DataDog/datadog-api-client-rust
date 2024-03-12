// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of active NPM hosts for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageNetworkHostsHour {
    /// Contains the number of active NPM hosts.
    #[serde(
        rename = "host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
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

impl UsageNetworkHostsHour {
    pub fn new() -> UsageNetworkHostsHour {
        UsageNetworkHostsHour {
            host_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn host_count(mut self, value: Option<i64>) -> Self {
        self.host_count = Some(value);
        self
    }

    pub fn hour(mut self, value: String) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageNetworkHostsHour {
    fn default() -> Self {
        Self::new()
    }
}

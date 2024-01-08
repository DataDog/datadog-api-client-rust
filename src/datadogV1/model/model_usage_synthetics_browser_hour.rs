// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of Synthetics Browser tests run for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSyntheticsBrowserHour {
    /// Contains the number of Synthetics Browser tests run.
    #[serde(
        rename = "browser_check_calls_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub browser_check_calls_count: Option<Option<i64>>,
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

impl UsageSyntheticsBrowserHour {
    pub fn new() -> UsageSyntheticsBrowserHour {
        UsageSyntheticsBrowserHour {
            browser_check_calls_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}
impl Default for UsageSyntheticsBrowserHour {
    fn default() -> Self {
        Self::new()
    }
}

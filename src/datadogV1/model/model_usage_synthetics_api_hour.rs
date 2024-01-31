// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of Synthetics API tests run for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSyntheticsAPIHour {
    /// Contains the number of Synthetics API tests run.
    #[serde(
        rename = "check_calls_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub check_calls_count: Option<Option<i64>>,
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

impl UsageSyntheticsAPIHour {
    pub fn new() -> UsageSyntheticsAPIHour {
        UsageSyntheticsAPIHour {
            check_calls_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn with_check_calls_count(&mut self, value: Option<i64>) -> &mut Self {
        self.check_calls_count = Some(value);
        self
    }

    pub fn with_hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
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
}
impl Default for UsageSyntheticsAPIHour {
    fn default() -> Self {
        Self::new()
    }
}

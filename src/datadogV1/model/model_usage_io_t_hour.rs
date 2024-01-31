// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// IoT usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageIoTHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The total number of IoT devices during a given hour.
    #[serde(
        rename = "iot_device_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub iot_device_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageIoTHour {
    pub fn new() -> UsageIoTHour {
        UsageIoTHour {
            hour: None,
            iot_device_count: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn with_hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn with_iot_device_count(&mut self, value: Option<i64>) -> &mut Self {
        self.iot_device_count = Some(value);
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
impl Default for UsageIoTHour {
    fn default() -> Self {
        Self::new()
    }
}

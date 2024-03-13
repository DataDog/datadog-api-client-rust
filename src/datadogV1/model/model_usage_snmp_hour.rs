// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The number of SNMP devices for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSNMPHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Contains the number of SNMP devices.
    #[serde(
        rename = "snmp_devices",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub snmp_devices: Option<Option<i64>>,
}

impl UsageSNMPHour {
    pub fn new() -> UsageSNMPHour {
        UsageSNMPHour {
            hour: None,
            org_name: None,
            public_id: None,
            snmp_devices: None,
        }
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

    pub fn snmp_devices(mut self, value: Option<i64>) -> Self {
        self.snmp_devices = Some(value);
        self
    }
}

impl Default for UsageSNMPHour {
    fn default() -> Self {
        Self::new()
    }
}

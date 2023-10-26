// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IPAllowlistEntryType {
    #[serde(rename = "ip_allowlist_entry")]
    IP_ALLOWLIST_ENTRY,
}

impl ToString for IPAllowlistEntryType {
    fn to_string(&self) -> String {
        match self {
            Self::IP_ALLOWLIST_ENTRY => String::from("ip_allowlist_entry"),
        }
    }
}

impl Default for IPAllowlistEntryType {
    fn default() -> IPAllowlistEntryType {
        Self::IP_ALLOWLIST_ENTRY
    }
}

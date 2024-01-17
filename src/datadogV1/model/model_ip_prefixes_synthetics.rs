// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Available prefix information for the Synthetics endpoints.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPPrefixesSynthetics {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4")]
    pub prefixes_ipv4: Option<Vec<String>>,
    /// List of IPv4 prefixes by location.
    #[serde(rename = "prefixes_ipv4_by_location")]
    pub prefixes_ipv4_by_location: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6")]
    pub prefixes_ipv6: Option<Vec<String>>,
    /// List of IPv6 prefixes by location.
    #[serde(rename = "prefixes_ipv6_by_location")]
    pub prefixes_ipv6_by_location: Option<std::collections::BTreeMap<String, Vec<String>>>,
}

impl IPPrefixesSynthetics {
    pub fn new() -> IPPrefixesSynthetics {
        IPPrefixesSynthetics {
            prefixes_ipv4: None,
            prefixes_ipv4_by_location: None,
            prefixes_ipv6: None,
            prefixes_ipv6_by_location: None,
        }
    }
}
impl Default for IPPrefixesSynthetics {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Available prefix information for the Process endpoints.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPPrefixesProcess {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4")]
    pub prefixes_ipv4: Option<Vec<String>>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6")]
    pub prefixes_ipv6: Option<Vec<String>>,
}

impl IPPrefixesProcess {
    pub fn new() -> IPPrefixesProcess {
        IPPrefixesProcess {
            prefixes_ipv4: None,
            prefixes_ipv6: None,
        }
    }
}

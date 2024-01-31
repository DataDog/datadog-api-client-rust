// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Available prefix information for the Orchestrator endpoints.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPPrefixesOrchestrator {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4")]
    pub prefixes_ipv4: Option<Vec<String>>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6")]
    pub prefixes_ipv6: Option<Vec<String>>,
}

impl IPPrefixesOrchestrator {
    pub fn new() -> IPPrefixesOrchestrator {
        IPPrefixesOrchestrator {
            prefixes_ipv4: None,
            prefixes_ipv6: None,
        }
    }

    pub fn with_prefixes_ipv4(&mut self, value: Vec<String>) -> &mut Self {
        self.prefixes_ipv4 = Some(value);
        self
    }

    pub fn with_prefixes_ipv6(&mut self, value: Vec<String>) -> &mut Self {
        self.prefixes_ipv6 = Some(value);
        self
    }
}
impl Default for IPPrefixesOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

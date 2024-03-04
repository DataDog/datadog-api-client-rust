// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Third party integrations that Datadog supports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Integrations {
    /// PagerDuty service URL for the service.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<String>,
}

impl ServiceDefinitionV1Integrations {
    pub fn new() -> ServiceDefinitionV1Integrations {
        ServiceDefinitionV1Integrations { pagerduty: None }
    }

    pub fn pagerduty(mut self, value: String) -> Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV1Integrations {
    fn default() -> Self {
        Self::new()
    }
}

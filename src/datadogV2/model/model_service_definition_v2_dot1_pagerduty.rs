// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// PagerDuty integration for the service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1Pagerduty {
    /// PagerDuty service url.
    #[serde(rename = "service-url")]
    pub service_url: Option<String>,
}

impl ServiceDefinitionV2Dot1Pagerduty {
    pub fn new() -> ServiceDefinitionV2Dot1Pagerduty {
        ServiceDefinitionV2Dot1Pagerduty { service_url: None }
    }

    pub fn with_service_url(&mut self, value: String) -> &mut Self {
        self.service_url = Some(value);
        self
    }
}
impl Default for ServiceDefinitionV2Dot1Pagerduty {
    fn default() -> Self {
        Self::new()
    }
}

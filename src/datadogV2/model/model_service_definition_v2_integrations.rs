// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Third party integrations that Datadog supports.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Integrations {
    /// Opsgenie integration for the service.
    #[serde(rename = "opsgenie")]
    pub opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Opsgenie>,
    /// PagerDuty service URL for the service.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<String>,
}

impl ServiceDefinitionV2Integrations {
    pub fn new() -> ServiceDefinitionV2Integrations {
        ServiceDefinitionV2Integrations {
            opsgenie: None,
            pagerduty: None,
        }
    }

    pub fn opsgenie(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Opsgenie,
    ) -> &mut Self {
        self.opsgenie = Some(value);
        self
    }

    pub fn pagerduty(&mut self, value: String) -> &mut Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV2Integrations {
    fn default() -> Self {
        Self::new()
    }
}

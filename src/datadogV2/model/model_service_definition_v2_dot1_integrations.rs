// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Third party integrations that Datadog supports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1Integrations {
    /// Opsgenie integration for the service.
    #[serde(rename = "opsgenie")]
    pub opsgenie: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Opsgenie>,
    /// PagerDuty integration for the service.
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1Pagerduty>,
}

impl ServiceDefinitionV2Dot1Integrations {
    pub fn new() -> ServiceDefinitionV2Dot1Integrations {
        ServiceDefinitionV2Dot1Integrations {
            opsgenie: None,
            pagerduty: None,
        }
    }

    pub fn opsgenie(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Opsgenie,
    ) -> &mut Self {
        self.opsgenie = Some(value);
        self
    }

    pub fn pagerduty(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1Pagerduty,
    ) -> &mut Self {
        self.pagerduty = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV2Dot1Integrations {
    fn default() -> Self {
        Self::new()
    }
}

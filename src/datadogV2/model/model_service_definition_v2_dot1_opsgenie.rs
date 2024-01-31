// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Opsgenie integration for the service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1Opsgenie {
    /// Opsgenie instance region.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::ServiceDefinitionV2Dot1OpsgenieRegion>,
    /// Opsgenie service url.
    #[serde(rename = "service-url")]
    pub service_url: String,
}

impl ServiceDefinitionV2Dot1Opsgenie {
    pub fn new(service_url: String) -> ServiceDefinitionV2Dot1Opsgenie {
        ServiceDefinitionV2Dot1Opsgenie {
            region: None,
            service_url,
        }
    }

    pub fn with_region(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot1OpsgenieRegion,
    ) -> &mut Self {
        self.region = Some(value);
        self
    }
}

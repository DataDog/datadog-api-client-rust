// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Opsgenie integration for the service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot2Opsgenie {
    /// Opsgenie instance region.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion>,
    /// Opsgenie service url.
    #[serde(rename = "service-url")]
    pub service_url: String,
}

impl ServiceDefinitionV2Dot2Opsgenie {
    pub fn new(service_url: String) -> ServiceDefinitionV2Dot2Opsgenie {
        ServiceDefinitionV2Dot2Opsgenie {
            region: None,
            service_url,
        }
    }

    pub fn region(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionV2Dot2OpsgenieRegion,
    ) -> Self {
        self.region = Some(value);
        self
    }
}

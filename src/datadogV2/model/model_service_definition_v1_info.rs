// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Basic information about a service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Info {
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A friendly name of the service.
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    /// Service tier.
    #[serde(rename = "service-tier")]
    pub service_tier: Option<String>,
}

impl ServiceDefinitionV1Info {
    pub fn new(dd_service: String) -> ServiceDefinitionV1Info {
        ServiceDefinitionV1Info {
            dd_service,
            description: None,
            display_name: None,
            service_tier: None,
        }
    }

    pub fn with_description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn with_display_name(&mut self, value: String) -> &mut Self {
        self.display_name = Some(value);
        self
    }

    pub fn with_service_tier(&mut self, value: String) -> &mut Self {
        self.service_tier = Some(value);
        self
    }
}

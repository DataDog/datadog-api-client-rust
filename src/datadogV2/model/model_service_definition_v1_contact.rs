// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Contact information about the service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Contact {
    /// Service owner’s email.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Service owner’s Slack channel.
    #[serde(rename = "slack")]
    pub slack: Option<String>,
}

impl ServiceDefinitionV1Contact {
    pub fn new() -> ServiceDefinitionV1Contact {
        ServiceDefinitionV1Contact {
            email: None,
            slack: None,
        }
    }
}

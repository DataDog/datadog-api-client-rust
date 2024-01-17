// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Org related information about the service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Org {
    /// App feature this service supports.
    #[serde(rename = "application")]
    pub application: Option<String>,
    /// Team that owns the service.
    #[serde(rename = "team")]
    pub team: Option<String>,
}

impl ServiceDefinitionV1Org {
    pub fn new() -> ServiceDefinitionV1Org {
        ServiceDefinitionV1Org {
            application: None,
            team: None,
        }
    }
}

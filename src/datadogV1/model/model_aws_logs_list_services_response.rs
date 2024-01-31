// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The list of current AWS services for which Datadog offers automatic log collection.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsListServicesResponse {
    /// Key value in returned object.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of service available for configuration with Datadog logs.
    #[serde(rename = "label")]
    pub label: Option<String>,
}

impl AWSLogsListServicesResponse {
    pub fn new() -> AWSLogsListServicesResponse {
        AWSLogsListServicesResponse {
            id: None,
            label: None,
        }
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_label(&mut self, value: String) -> &mut Self {
        self.label = Some(value);
        self
    }
}
impl Default for AWSLogsListServicesResponse {
    fn default() -> Self {
        Self::new()
    }
}

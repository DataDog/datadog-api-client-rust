// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Schema validation warnings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionMetaWarnings {
    /// The warning instance location.
    #[serde(rename = "instance-location")]
    pub instance_location: Option<String>,
    /// The warning keyword location.
    #[serde(rename = "keyword-location")]
    pub keyword_location: Option<String>,
    /// The warning message.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl ServiceDefinitionMetaWarnings {
    pub fn new() -> ServiceDefinitionMetaWarnings {
        ServiceDefinitionMetaWarnings {
            instance_location: None,
            keyword_location: None,
            message: None,
        }
    }

    pub fn instance_location(&mut self, value: String) -> &mut Self {
        self.instance_location = Some(value);
        self
    }

    pub fn keyword_location(&mut self, value: String) -> &mut Self {
        self.keyword_location = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }
}

impl Default for ServiceDefinitionMetaWarnings {
    fn default() -> Self {
        Self::new()
    }
}

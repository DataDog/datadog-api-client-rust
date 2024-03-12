// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service definition data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionData {
    /// Service definition attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ServiceDefinitionDataAttributes>,
    /// Service definition id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Service definition type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl ServiceDefinitionData {
    pub fn new() -> ServiceDefinitionData {
        ServiceDefinitionData {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::ServiceDefinitionDataAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ServiceDefinitionData {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Container Image object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImage {
    /// Attributes for a Container Image.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ContainerImageAttributes>,
    /// Container Image ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of Container Image.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerImageType>,
}

impl ContainerImage {
    pub fn new() -> ContainerImage {
        ContainerImage {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::ContainerImageAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::ContainerImageType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ContainerImage {
    fn default() -> Self {
        Self::new()
    }
}

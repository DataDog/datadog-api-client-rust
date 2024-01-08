// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Container group object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroup {
    /// Attributes for a container group.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::ContainerGroupAttributes>>,
    /// Container Group ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships to containers inside a container group.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::ContainerGroupRelationships>>,
    /// Type of container group.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerGroupType>,
}

impl ContainerGroup {
    pub fn new() -> ContainerGroup {
        ContainerGroup {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for ContainerGroup {
    fn default() -> Self {
        Self::new()
    }
}

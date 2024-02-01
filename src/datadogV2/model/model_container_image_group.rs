// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Container Image Group object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageGroup {
    /// Attributes for a Container Image Group.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ContainerImageGroupAttributes>,
    /// Container Image Group ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships inside a Container Image Group.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::ContainerImageGroupRelationships>,
    /// Type of Container Image Group.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerImageGroupType>,
}

impl ContainerImageGroup {
    pub fn new() -> ContainerImageGroup {
        ContainerImageGroup {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::ContainerImageGroupAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::ContainerImageGroupRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::ContainerImageGroupType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ContainerImageGroup {
    fn default() -> Self {
        Self::new()
    }
}

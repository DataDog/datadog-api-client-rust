// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships to containers inside a container group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupRelationships {
    /// Relationships to Containers inside a Container Group.
    #[serde(rename = "containers")]
    pub containers: Option<crate::datadogV2::model::ContainerGroupRelationshipsLink>,
}

impl ContainerGroupRelationships {
    pub fn new() -> ContainerGroupRelationships {
        ContainerGroupRelationships { containers: None }
    }

    pub fn containers(
        &mut self,
        value: crate::datadogV2::model::ContainerGroupRelationshipsLink,
    ) -> &mut Self {
        self.containers = Some(value);
        self
    }
}

impl Default for ContainerGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}

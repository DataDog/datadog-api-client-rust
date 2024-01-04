// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships inside a Container Image Group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageGroupRelationships {
    /// Relationships to Container Images inside a Container Image Group.
    #[serde(rename = "container_images")]
    pub container_images:
        Option<Box<crate::datadogV2::model::ContainerImageGroupImagesRelationshipsLink>>,
}

impl ContainerImageGroupRelationships {
    pub fn new() -> ContainerImageGroupRelationships {
        ContainerImageGroupRelationships {
            container_images: None,
        }
    }
}
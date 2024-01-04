// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships to Container Images inside a Container Image Group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageGroupImagesRelationshipsLink {
    /// Links data.
    #[serde(rename = "data")]
    pub data: Option<Vec<String>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::ContainerImageGroupRelationshipsLinks>>,
}

impl ContainerImageGroupImagesRelationshipsLink {
    pub fn new() -> ContainerImageGroupImagesRelationshipsLink {
        ContainerImageGroupImagesRelationshipsLink {
            data: None,
            links: None,
        }
    }
}
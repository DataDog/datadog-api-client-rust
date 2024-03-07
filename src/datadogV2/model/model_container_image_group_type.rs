// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ContainerImageGroupType {
    #[serde(rename = "container_image_group")]
    CONTAINER_IMAGE_GROUP,
}
impl ToString for ContainerImageGroupType {
    fn to_string(&self) -> String {
        match self {
            Self::CONTAINER_IMAGE_GROUP => String::from("container_image_group"),
        }
    }
}

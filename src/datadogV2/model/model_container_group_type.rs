// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContainerGroupType {
    #[serde(rename = "container_group")]
    CONTAINER_GROUP,
}

impl ToString for ContainerGroupType {
    fn to_string(&self) -> String {
        match self {
            Self::CONTAINER_GROUP => String::from("container_group"),
        }
    }
}
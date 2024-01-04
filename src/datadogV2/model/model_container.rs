// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Container object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    /// Attributes for a container.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::ContainerAttributes>>,
    /// Container ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of container.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerType>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}

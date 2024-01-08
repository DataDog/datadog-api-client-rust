// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Links attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageGroupRelationshipsLinks {
    /// Link to related Container Images.
    #[serde(rename = "related")]
    pub related: Option<String>,
}

impl ContainerImageGroupRelationshipsLinks {
    pub fn new() -> ContainerImageGroupRelationshipsLinks {
        ContainerImageGroupRelationshipsLinks { related: None }
    }
}
impl Default for ContainerImageGroupRelationshipsLinks {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Links attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerGroupRelationshipsLinks {
    /// Link to related containers.
    #[serde(rename = "related")]
    pub related: Option<String>,
}

impl ContainerGroupRelationshipsLinks {
    pub fn new() -> ContainerGroupRelationshipsLinks {
        ContainerGroupRelationshipsLinks { related: None }
    }
}
// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for a Container Image Group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageGroupAttributes {
    /// Number of Container Images in the group.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Name of the Container Image group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Tags from the group name parsed in key/value format.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl ContainerImageGroupAttributes {
    pub fn new() -> ContainerImageGroupAttributes {
        ContainerImageGroupAttributes {
            count: None,
            name: None,
            tags: None,
        }
    }

    pub fn count(&mut self, value: i64) -> &mut Self {
        self.count = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn tags(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl Default for ContainerImageGroupAttributes {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountResourceAttributes {
    /// Enable the `custom.consumer_lag_offset` metric, which contains extra metric tags.
    #[serde(rename = "enable_custom_metrics", skip_serializing_if = "Option::is_none")]
    pub enable_custom_metrics: Option<bool>,
    /// The ID associated with a Confluent resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The resource type of the Resource. Can be `kafka`, `connector`, `ksql`, or `schema_registry`.
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ConfluentAccountResourceAttributes {
    /// Attributes object for updating a Confluent resource.
    pub fn new(resource_type: String) -> ConfluentAccountResourceAttributes {
        ConfluentAccountResourceAttributes {
            enable_custom_metrics: None,
            id: None,
            resource_type: resource_type,
            tags: None,
        }
    }
}

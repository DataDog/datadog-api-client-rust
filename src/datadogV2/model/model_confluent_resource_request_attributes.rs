// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes object for updating a Confluent resource.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceRequestAttributes {
    /// Enable the `custom.consumer_lag_offset` metric, which contains extra metric tags.
    #[serde(rename = "enable_custom_metrics")]
    pub enable_custom_metrics: Option<bool>,
    /// The resource type of the Resource. Can be `kafka`, `connector`, `ksql`, or `schema_registry`.
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ConfluentResourceRequestAttributes {
    pub fn new(resource_type: String) -> ConfluentResourceRequestAttributes {
        ConfluentResourceRequestAttributes {
            enable_custom_metrics: None,
            resource_type,
            tags: None,
        }
    }
}

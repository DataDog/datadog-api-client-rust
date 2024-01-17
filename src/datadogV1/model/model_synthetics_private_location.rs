// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing information about the private location to create.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocation {
    /// Description of the private location.
    #[serde(rename = "description")]
    pub description: String,
    /// Unique identifier of the private location.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Object containing metadata about the private location.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::SyntheticsPrivateLocationMetadata>>,
    /// Name of the private location.
    #[serde(rename = "name")]
    pub name: String,
    /// Secrets for the private location. Only present in the response when creating the private location.
    #[serde(rename = "secrets")]
    pub secrets: Option<Box<crate::datadogV1::model::SyntheticsPrivateLocationSecrets>>,
    /// Array of tags attached to the private location.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl SyntheticsPrivateLocation {
    pub fn new(description: String, name: String, tags: Vec<String>) -> SyntheticsPrivateLocation {
        SyntheticsPrivateLocation {
            description,
            id: None,
            metadata: None,
            name,
            secrets: None,
            tags,
        }
    }
}

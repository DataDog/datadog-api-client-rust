// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceAttributes {
    /// A list of tags for the Fastly service.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FastlyServiceAttributes {
    /// Attributes object for Fastly service requests.
    pub fn new() -> FastlyServiceAttributes {
        FastlyServiceAttributes { tags: None }
    }
}
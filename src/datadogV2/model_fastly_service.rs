// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyService {
    /// The id of the Fastly service
    #[serde(rename = "id")]
    pub id: String,
    /// A list of tags for the Fastly service.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FastlyService {
    /// The schema representation of a Fastly service.
    pub fn new(id: String) -> FastlyService {
        FastlyService { id: id, tags: None }
    }
}

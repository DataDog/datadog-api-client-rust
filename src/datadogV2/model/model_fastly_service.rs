// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The schema representation of a Fastly service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyService {
    /// The id of the Fastly service
    #[serde(rename = "id")]
    pub id: String,
    /// A list of tags for the Fastly service.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl FastlyService {
    pub fn new(id: String) -> FastlyService {
        FastlyService { id, tags: None }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

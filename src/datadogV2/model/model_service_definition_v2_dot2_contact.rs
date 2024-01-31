// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service owner's contacts information.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot2Contact {
    /// Contact value.
    #[serde(rename = "contact")]
    pub contact: String,
    /// Contact Name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Contact type. Datadog recognizes the following types: `email`, `slack`, and `microsoft-teams`.
    #[serde(rename = "type")]
    pub type_: String,
}

impl ServiceDefinitionV2Dot2Contact {
    pub fn new(contact: String, type_: String) -> ServiceDefinitionV2Dot2Contact {
        ServiceDefinitionV2Dot2Contact {
            contact,
            name: None,
            type_,
        }
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

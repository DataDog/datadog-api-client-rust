// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the creator of the shared element.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    /// Email of the creator.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the creator.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Name of the creator.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
}

impl Creator {
    pub fn new() -> Creator {
        Creator {
            email: None,
            handle: None,
            name: None,
        }
    }

    pub fn email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn name(&mut self, value: Option<String>) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for Creator {
    fn default() -> Self {
        Self::new()
    }
}

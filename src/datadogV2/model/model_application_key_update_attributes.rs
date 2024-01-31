// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes used to update an application Key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyUpdateAttributes {
    /// Name of the application key.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option")]
    pub scopes: Option<Option<Vec<String>>>,
}

impl ApplicationKeyUpdateAttributes {
    pub fn new() -> ApplicationKeyUpdateAttributes {
        ApplicationKeyUpdateAttributes {
            name: None,
            scopes: None,
        }
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_scopes(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.scopes = Some(value);
        self
    }
}
impl Default for ApplicationKeyUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

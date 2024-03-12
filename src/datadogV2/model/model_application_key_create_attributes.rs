// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes used to create an application Key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyCreateAttributes {
    /// Name of the application key.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option")]
    pub scopes: Option<Option<Vec<String>>>,
}

impl ApplicationKeyCreateAttributes {
    pub fn new(name: String) -> ApplicationKeyCreateAttributes {
        ApplicationKeyCreateAttributes { name, scopes: None }
    }

    pub fn scopes(mut self, value: Option<Vec<String>>) -> Self {
        self.scopes = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the created user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountCreateAttributes {
    /// The email of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// The name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether the user is a service account. Must be true.
    #[serde(rename = "service_account")]
    pub service_account: bool,
    /// The title of the user.
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl ServiceAccountCreateAttributes {
    pub fn new(email: String, service_account: bool) -> ServiceAccountCreateAttributes {
        ServiceAccountCreateAttributes {
            email,
            name: None,
            service_account,
            title: None,
        }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}

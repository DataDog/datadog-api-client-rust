// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// User who shared the dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardAuthor {
    /// Identifier of the user who shared the dashboard.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Name of the user who shared the dashboard.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
}

impl SharedDashboardAuthor {
    pub fn new() -> SharedDashboardAuthor {
        SharedDashboardAuthor {
            handle: None,
            name: None,
        }
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for SharedDashboardAuthor {
    fn default() -> Self {
        Self::new()
    }
}

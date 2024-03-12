// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Additional information related to your service account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPServiceAccountMeta {
    /// The current list of projects accessible from your service account.
    #[serde(rename = "accessible_projects")]
    pub accessible_projects: Option<Vec<String>>,
}

impl GCPServiceAccountMeta {
    pub fn new() -> GCPServiceAccountMeta {
        GCPServiceAccountMeta {
            accessible_projects: None,
        }
    }

    pub fn accessible_projects(mut self, value: Vec<String>) -> Self {
        self.accessible_projects = Some(value);
        self
    }
}

impl Default for GCPServiceAccountMeta {
    fn default() -> Self {
        Self::new()
    }
}

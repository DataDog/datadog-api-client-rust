// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GCPServiceAccountMeta {
    /// The current list of projects accessible from your service account.
    #[serde(rename = "accessible_projects", skip_serializing_if = "Option::is_none")]
    pub accessible_projects: Option<Vec<String>>,
}

impl GCPServiceAccountMeta {
    /// Additional information related to your service account.
    pub fn new() -> GCPServiceAccountMeta {
        GCPServiceAccountMeta {
            accessible_projects: None,
        }
    }
}

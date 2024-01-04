// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata for the Jira integration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraIntegrationMetadata {
    /// Array of Jira issues in this integration metadata.
    #[serde(rename = "issues")]
    pub issues: Vec<crate::datadogV2::model::JiraIntegrationMetadataIssuesItem>,
}

impl JiraIntegrationMetadata {
    pub fn new(
        issues: Vec<crate::datadogV2::model::JiraIntegrationMetadataIssuesItem>,
    ) -> JiraIntegrationMetadata {
        JiraIntegrationMetadata { issues }
    }
}

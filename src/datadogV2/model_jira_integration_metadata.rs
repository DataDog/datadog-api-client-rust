// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraIntegrationMetadata {
    /// Array of Jira issues in this integration metadata.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Vec<JiraIntegrationMetadataIssuesItem>,
}


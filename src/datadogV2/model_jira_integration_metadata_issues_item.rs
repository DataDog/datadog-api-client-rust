// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraIntegrationMetadataIssuesItem {
    /// URL of issue's Jira account.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: String,
    /// Jira issue's issue key.
    #[serde(rename = "issue_key", skip_serializing_if = "Option::is_none")]
    pub issue_key: String,
    /// Jira issue's issue type.
    #[serde(rename = "issuetype_id", skip_serializing_if = "Option::is_none")]
    pub issuetype_id: String,
    /// Jira issue's project keys.
    #[serde(rename = "project_key", skip_serializing_if = "Option::is_none")]
    pub project_key: String,
    /// URL redirecting to the Jira issue.
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: String,
}


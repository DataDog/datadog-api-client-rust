// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Item in the Jira integration metadata issue array.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraIntegrationMetadataIssuesItem {
    /// URL of issue's Jira account.
    #[serde(rename = "account")]
    pub account: String,
    /// Jira issue's issue key.
    #[serde(rename = "issue_key")]
    pub issue_key: Option<String>,
    /// Jira issue's issue type.
    #[serde(rename = "issuetype_id")]
    pub issuetype_id: Option<String>,
    /// Jira issue's project keys.
    #[serde(rename = "project_key")]
    pub project_key: String,
    /// URL redirecting to the Jira issue.
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
}

impl JiraIntegrationMetadataIssuesItem {
    pub fn new(account: String, project_key: String) -> JiraIntegrationMetadataIssuesItem {
        JiraIntegrationMetadataIssuesItem {
            account,
            issue_key: None,
            issuetype_id: None,
            project_key,
            redirect_url: None,
        }
    }

    pub fn issue_key(&mut self, value: String) -> &mut Self {
        self.issue_key = Some(value);
        self
    }

    pub fn issuetype_id(&mut self, value: String) -> &mut Self {
        self.issuetype_id = Some(value);
        self
    }

    pub fn redirect_url(&mut self, value: String) -> &mut Self {
        self.redirect_url = Some(value);
        self
    }
}

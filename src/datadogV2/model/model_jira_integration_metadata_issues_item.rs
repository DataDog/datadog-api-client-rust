// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Item in the Jira integration metadata issue array.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIntegrationMetadataIssuesItem {
    pub fn new(account: String, project_key: String) -> JiraIntegrationMetadataIssuesItem {
        JiraIntegrationMetadataIssuesItem {
            account,
            issue_key: None,
            issuetype_id: None,
            project_key,
            redirect_url: None,
            _unparsed: false,
        }
    }

    pub fn issue_key(mut self, value: String) -> Self {
        self.issue_key = Some(value);
        self
    }

    pub fn issuetype_id(mut self, value: String) -> Self {
        self.issuetype_id = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: String) -> Self {
        self.redirect_url = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for JiraIntegrationMetadataIssuesItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIntegrationMetadataIssuesItemVisitor;
        impl<'a> Visitor<'a> for JiraIntegrationMetadataIssuesItemVisitor {
            type Value = JiraIntegrationMetadataIssuesItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account: Option<String> = None;
                let mut issue_key: Option<String> = None;
                let mut issuetype_id: Option<String> = None;
                let mut project_key: Option<String> = None;
                let mut redirect_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account" => {
                            account = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_key" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuetype_id" => {
                            if v.is_null() {
                                continue;
                            }
                            issuetype_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_key" => {
                            project_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_url" => {
                            if v.is_null() {
                                continue;
                            }
                            redirect_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account = account.ok_or_else(|| M::Error::missing_field("account"))?;
                let project_key =
                    project_key.ok_or_else(|| M::Error::missing_field("project_key"))?;

                let content = JiraIntegrationMetadataIssuesItem {
                    account,
                    issue_key,
                    issuetype_id,
                    project_key,
                    redirect_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIntegrationMetadataIssuesItemVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Jira issue information
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIssueResult {
    /// Jira issue ID
    #[serde(rename = "issue_id")]
    pub issue_id: Option<String>,
    /// Jira issue key
    #[serde(rename = "issue_key")]
    pub issue_key: Option<String>,
    /// Jira issue URL
    #[serde(rename = "issue_url")]
    pub issue_url: Option<String>,
    /// Jira project key
    #[serde(rename = "project_key")]
    pub project_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueResult {
    pub fn new() -> JiraIssueResult {
        JiraIssueResult {
            issue_id: None,
            issue_key: None,
            issue_url: None,
            project_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn issue_id(mut self, value: String) -> Self {
        self.issue_id = Some(value);
        self
    }

    pub fn issue_key(mut self, value: String) -> Self {
        self.issue_key = Some(value);
        self
    }

    pub fn issue_url(mut self, value: String) -> Self {
        self.issue_url = Some(value);
        self
    }

    pub fn project_key(mut self, value: String) -> Self {
        self.project_key = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for JiraIssueResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for JiraIssueResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueResultVisitor;
        impl<'a> Visitor<'a> for JiraIssueResultVisitor {
            type Value = JiraIssueResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut issue_id: Option<String> = None;
                let mut issue_key: Option<String> = None;
                let mut issue_url: Option<String> = None;
                let mut project_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "issue_id" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_key" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_url" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_key" => {
                            if v.is_null() {
                                continue;
                            }
                            project_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = JiraIssueResult {
                    issue_id,
                    issue_key,
                    issue_url,
                    project_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueResultVisitor)
    }
}

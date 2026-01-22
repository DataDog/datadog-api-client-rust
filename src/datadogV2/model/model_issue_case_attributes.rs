// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of a case.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueCaseAttributes {
    /// Timestamp of when the case was archived.
    #[serde(rename = "archived_at")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the case was closed.
    #[serde(rename = "closed_at")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the case was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Source of the case creation.
    #[serde(rename = "creation_source")]
    pub creation_source: Option<String>,
    /// Description of the case.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Due date of the case.
    #[serde(rename = "due_date")]
    pub due_date: Option<String>,
    /// Insights of the case.
    #[serde(rename = "insights")]
    pub insights: Option<Vec<crate::datadogV2::model::IssueCaseInsight>>,
    /// Jira issue of the case.
    #[serde(rename = "jira_issue")]
    pub jira_issue: Option<crate::datadogV2::model::IssueCaseJiraIssue>,
    /// Key of the case.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Timestamp of when the case was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Case priority
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::CasePriority>,
    /// Deprecated way of representing the case status, which only supports OPEN, IN_PROGRESS, and CLOSED statuses. Use `status_name` instead.
    #[deprecated]
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CaseStatus>,
    /// Title of the case.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the case.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueCaseAttributes {
    pub fn new() -> IssueCaseAttributes {
        #[allow(deprecated)]
        IssueCaseAttributes {
            archived_at: None,
            closed_at: None,
            created_at: None,
            creation_source: None,
            description: None,
            due_date: None,
            insights: None,
            jira_issue: None,
            key: None,
            modified_at: None,
            priority: None,
            status: None,
            title: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn archived_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.archived_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn closed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.closed_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn creation_source(mut self, value: String) -> Self {
        self.creation_source = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn due_date(mut self, value: String) -> Self {
        self.due_date = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn insights(mut self, value: Vec<crate::datadogV2::model::IssueCaseInsight>) -> Self {
        self.insights = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn jira_issue(mut self, value: crate::datadogV2::model::IssueCaseJiraIssue) -> Self {
        self.jira_issue = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn priority(mut self, value: crate::datadogV2::model::CasePriority) -> Self {
        self.priority = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status(mut self, value: crate::datadogV2::model::CaseStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for IssueCaseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueCaseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueCaseAttributesVisitor;
        impl<'a> Visitor<'a> for IssueCaseAttributesVisitor {
            type Value = IssueCaseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut closed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creation_source: Option<String> = None;
                let mut description: Option<String> = None;
                let mut due_date: Option<String> = None;
                let mut insights: Option<Vec<crate::datadogV2::model::IssueCaseInsight>> = None;
                let mut jira_issue: Option<crate::datadogV2::model::IssueCaseJiraIssue> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut priority: Option<crate::datadogV2::model::CasePriority> = None;
                let mut status: Option<crate::datadogV2::model::CaseStatus> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            if v.is_null() {
                                continue;
                            }
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "closed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            closed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_source" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "due_date" => {
                            if v.is_null() {
                                continue;
                            }
                            due_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "insights" => {
                            if v.is_null() {
                                continue;
                            }
                            insights = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_issue" => {
                            if v.is_null() {
                                continue;
                            }
                            jira_issue = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::CasePriority::UnparsedObject(
                                        _priority,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CaseStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = IssueCaseAttributes {
                    archived_at,
                    closed_at,
                    created_at,
                    creation_source,
                    description,
                    due_date,
                    insights,
                    jira_issue,
                    key,
                    modified_at,
                    priority,
                    status,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueCaseAttributesVisitor)
    }
}

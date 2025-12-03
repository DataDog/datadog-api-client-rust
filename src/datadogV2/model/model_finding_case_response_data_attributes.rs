// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the case.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FindingCaseResponseDataAttributes {
    /// Timestamp of when the case was archived.
    #[serde(rename = "archived_at")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Relationship to user.
    #[serde(rename = "assigned_to")]
    pub assigned_to: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, Vec<String>>>,
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
    pub insights: Option<Vec<crate::datadogV2::model::CaseInsightsItems>>,
    /// Jira issue associated with the case.
    #[serde(rename = "jira_issue")]
    pub jira_issue: Option<crate::datadogV2::model::FindingJiraIssue>,
    /// Key of the case.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Timestamp of when the case was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Priority of the case.
    #[serde(rename = "priority")]
    pub priority: Option<String>,
    /// Status of the case.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Status group of the case.
    #[serde(rename = "status_group")]
    pub status_group: Option<String>,
    /// Status name of the case.
    #[serde(rename = "status_name")]
    pub status_name: Option<String>,
    /// Title of the case.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the case. For security cases, this is always "SECURITY".
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FindingCaseResponseDataAttributes {
    pub fn new() -> FindingCaseResponseDataAttributes {
        FindingCaseResponseDataAttributes {
            archived_at: None,
            assigned_to: None,
            attributes: None,
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
            status_group: None,
            status_name: None,
            title: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn assigned_to(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn attributes(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn closed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creation_source(mut self, value: String) -> Self {
        self.creation_source = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn due_date(mut self, value: String) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn insights(mut self, value: Vec<crate::datadogV2::model::CaseInsightsItems>) -> Self {
        self.insights = Some(value);
        self
    }

    pub fn jira_issue(mut self, value: crate::datadogV2::model::FindingJiraIssue) -> Self {
        self.jira_issue = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn priority(mut self, value: String) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_group(mut self, value: String) -> Self {
        self.status_group = Some(value);
        self
    }

    pub fn status_name(mut self, value: String) -> Self {
        self.status_name = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

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

impl Default for FindingCaseResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FindingCaseResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FindingCaseResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for FindingCaseResponseDataAttributesVisitor {
            type Value = FindingCaseResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut assigned_to: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut attributes: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut closed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creation_source: Option<String> = None;
                let mut description: Option<String> = None;
                let mut due_date: Option<String> = None;
                let mut insights: Option<Vec<crate::datadogV2::model::CaseInsightsItems>> = None;
                let mut jira_issue: Option<crate::datadogV2::model::FindingJiraIssue> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut priority: Option<String> = None;
                let mut status: Option<String> = None;
                let mut status_group: Option<String> = None;
                let mut status_name: Option<String> = None;
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
                        "assigned_to" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_to =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_group" => {
                            if v.is_null() {
                                continue;
                            }
                            status_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_name" => {
                            if v.is_null() {
                                continue;
                            }
                            status_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FindingCaseResponseDataAttributes {
                    archived_at,
                    assigned_to,
                    attributes,
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
                    status_group,
                    status_name,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FindingCaseResponseDataAttributesVisitor)
    }
}

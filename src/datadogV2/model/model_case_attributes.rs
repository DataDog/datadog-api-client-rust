// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseAttributes {
    /// Timestamp of when the case was archived
    #[serde(
        rename = "archived_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived_at: Option<Option<String>>,
    /// Timestamp of when the case was closed
    #[serde(
        rename = "closed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub closed_at: Option<Option<String>>,
    /// Timestamp of when the case was created
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Jira issue attached to case
    #[serde(
        rename = "jira_issue",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub jira_issue: Option<Option<crate::datadogV2::model::JiraIssue>>,
    /// Key
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Timestamp of when the case was last modified
    #[serde(
        rename = "modified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_at: Option<Option<String>>,
    /// Case priority
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::CasePriority>,
    /// ServiceNow ticket attached to case
    #[serde(
        rename = "service_now_ticket",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub service_now_ticket: Option<Option<crate::datadogV2::model::ServiceNowTicket>>,
    /// Case status
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CaseStatus>,
    /// Title
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Case type
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CaseType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseAttributes {
    pub fn new() -> CaseAttributes {
        CaseAttributes {
            archived_at: None,
            closed_at: None,
            created_at: None,
            description: None,
            jira_issue: None,
            key: None,
            modified_at: None,
            priority: None,
            service_now_ticket: None,
            status: None,
            title: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: Option<String>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn closed_at(mut self, value: Option<String>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn jira_issue(mut self, value: Option<crate::datadogV2::model::JiraIssue>) -> Self {
        self.jira_issue = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn modified_at(mut self, value: Option<String>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn priority(mut self, value: crate::datadogV2::model::CasePriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn service_now_ticket(
        mut self,
        value: Option<crate::datadogV2::model::ServiceNowTicket>,
    ) -> Self {
        self.service_now_ticket = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::CaseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::CaseType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for CaseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseAttributesVisitor;
        impl<'a> Visitor<'a> for CaseAttributesVisitor {
            type Value = CaseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<Option<String>> = None;
                let mut closed_at: Option<Option<String>> = None;
                let mut created_at: Option<String> = None;
                let mut description: Option<String> = None;
                let mut jira_issue: Option<Option<crate::datadogV2::model::JiraIssue>> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<Option<String>> = None;
                let mut priority: Option<crate::datadogV2::model::CasePriority> = None;
                let mut service_now_ticket: Option<
                    Option<crate::datadogV2::model::ServiceNowTicket>,
                > = None;
                let mut status: Option<crate::datadogV2::model::CaseStatus> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CaseType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "closed_at" => {
                            closed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_issue" => {
                            jira_issue = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
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
                        "service_now_ticket" => {
                            service_now_ticket =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CaseType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = CaseAttributes {
                    archived_at,
                    closed_at,
                    created_at,
                    description,
                    jira_issue,
                    key,
                    modified_at,
                    priority,
                    service_now_ticket,
                    status,
                    title,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseAttributesVisitor)
    }
}

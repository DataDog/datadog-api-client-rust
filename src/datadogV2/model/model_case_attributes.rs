// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case resource attributes
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
    pub archived_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The definition of `CaseObjectAttributes` object.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// Timestamp of when the case was closed
    #[serde(
        rename = "closed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub closed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Timestamp of when the case was created
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Case custom attributes
    #[serde(rename = "custom_attributes")]
    pub custom_attributes:
        Option<std::collections::BTreeMap<String, crate::datadogV2::model::CustomAttributeValue>>,
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
    pub modified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
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
    /// Deprecated way of representing the case status, which only supports OPEN, IN_PROGRESS, and CLOSED statuses. Use `status_name` instead.
    #[deprecated]
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CaseStatus>,
    /// Status group of the case.
    #[serde(rename = "status_group")]
    pub status_group: Option<crate::datadogV2::model::CaseStatusGroup>,
    /// Status of the case. Must be one of the existing statuses for the case's type.
    #[serde(rename = "status_name")]
    pub status_name: Option<String>,
    /// Title
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Case type
    #[deprecated]
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CaseType>,
    /// Case type UUID
    #[serde(rename = "type_id")]
    pub type_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseAttributes {
    pub fn new() -> CaseAttributes {
        #[allow(deprecated)]
        CaseAttributes {
            archived_at: None,
            attributes: None,
            closed_at: None,
            created_at: None,
            custom_attributes: None,
            description: None,
            jira_issue: None,
            key: None,
            modified_at: None,
            priority: None,
            service_now_ticket: None,
            status: None,
            status_group: None,
            status_name: None,
            title: None,
            type_: None,
            type_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn archived_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.archived_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn attributes(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.attributes = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn closed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.closed_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_attributes(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::CustomAttributeValue>,
    ) -> Self {
        self.custom_attributes = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn jira_issue(mut self, value: Option<crate::datadogV2::model::JiraIssue>) -> Self {
        self.jira_issue = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn modified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.modified_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn priority(mut self, value: crate::datadogV2::model::CasePriority) -> Self {
        self.priority = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn service_now_ticket(
        mut self,
        value: Option<crate::datadogV2::model::ServiceNowTicket>,
    ) -> Self {
        self.service_now_ticket = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status(mut self, value: crate::datadogV2::model::CaseStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_group(mut self, value: crate::datadogV2::model::CaseStatusGroup) -> Self {
        self.status_group = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_name(mut self, value: String) -> Self {
        self.status_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn type_(mut self, value: crate::datadogV2::model::CaseType) -> Self {
        self.type_ = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn type_id(mut self, value: String) -> Self {
        self.type_id = Some(value);
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
                let mut archived_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut attributes: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut closed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut custom_attributes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::CustomAttributeValue,
                    >,
                > = None;
                let mut description: Option<String> = None;
                let mut jira_issue: Option<Option<crate::datadogV2::model::JiraIssue>> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut priority: Option<crate::datadogV2::model::CasePriority> = None;
                let mut service_now_ticket: Option<
                    Option<crate::datadogV2::model::ServiceNowTicket>,
                > = None;
                let mut status: Option<crate::datadogV2::model::CaseStatus> = None;
                let mut status_group: Option<crate::datadogV2::model::CaseStatusGroup> = None;
                let mut status_name: Option<String> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CaseType> = None;
                let mut type_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "custom_attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_attributes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "status_group" => {
                            if v.is_null() {
                                continue;
                            }
                            status_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status_group) = status_group {
                                match _status_group {
                                    crate::datadogV2::model::CaseStatusGroup::UnparsedObject(
                                        _status_group,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CaseType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            type_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = CaseAttributes {
                    archived_at,
                    attributes,
                    closed_at,
                    created_at,
                    custom_attributes,
                    description,
                    jira_issue,
                    key,
                    modified_at,
                    priority,
                    service_now_ticket,
                    status,
                    status_group,
                    status_name,
                    title,
                    type_,
                    type_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseAttributesVisitor)
    }
}

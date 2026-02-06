// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIssueDataAttributesRequest {
    /// Jira account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Custom fields for the Jira issue.
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option")]
    pub fields: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Jira issue type name.
    #[serde(rename = "issue_type")]
    pub issue_type: String,
    /// Jira issue type identifier.
    #[serde(rename = "issuetype_id")]
    pub issuetype_id: String,
    /// Mode for creating the Jira issue. Can be "single" or "multiple".
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV2::model::JiraIssueDataAttributesRequestMode>,
    /// Jira project identifier.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Jira project key.
    #[serde(rename = "project_key")]
    pub project_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueDataAttributesRequest {
    pub fn new(
        account_id: String,
        issue_type: String,
        issuetype_id: String,
        project_id: String,
        project_key: String,
    ) -> JiraIssueDataAttributesRequest {
        JiraIssueDataAttributesRequest {
            account_id,
            fields: None,
            issue_type,
            issuetype_id,
            mode: None,
            project_id,
            project_key,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn mode(
        mut self,
        value: crate::datadogV2::model::JiraIssueDataAttributesRequestMode,
    ) -> Self {
        self.mode = Some(value);
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

impl<'de> Deserialize<'de> for JiraIssueDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for JiraIssueDataAttributesRequestVisitor {
            type Value = JiraIssueDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut fields: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut issue_type: Option<String> = None;
                let mut issuetype_id: Option<String> = None;
                let mut mode: Option<crate::datadogV2::model::JiraIssueDataAttributesRequestMode> =
                    None;
                let mut project_id: Option<String> = None;
                let mut project_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type" => {
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuetype_id" => {
                            issuetype_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::JiraIssueDataAttributesRequestMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_key" => {
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let issue_type = issue_type.ok_or_else(|| M::Error::missing_field("issue_type"))?;
                let issuetype_id =
                    issuetype_id.ok_or_else(|| M::Error::missing_field("issuetype_id"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let project_key =
                    project_key.ok_or_else(|| M::Error::missing_field("project_key"))?;

                let content = JiraIssueDataAttributesRequest {
                    account_id,
                    fields,
                    issue_type,
                    issuetype_id,
                    mode,
                    project_id,
                    project_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueDataAttributesRequestVisitor)
    }
}

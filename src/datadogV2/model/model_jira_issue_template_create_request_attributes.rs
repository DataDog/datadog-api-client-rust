// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Jira issue template
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIssueTemplateCreateRequestAttributes {
    /// Custom fields for the Jira issue template
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The ID of the Jira issue type
    #[serde(rename = "issue_type_id")]
    pub issue_type_id: Option<String>,
    /// Reference to the Jira account
    #[serde(rename = "jira-account")]
    pub jira_account:
        Option<crate::datadogV2::model::JiraIssueTemplateCreateRequestAttributesJiraAccount>,
    /// The name of the issue template
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The ID of the Jira project
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueTemplateCreateRequestAttributes {
    pub fn new() -> JiraIssueTemplateCreateRequestAttributes {
        JiraIssueTemplateCreateRequestAttributes {
            fields: None,
            issue_type_id: None,
            jira_account: None,
            name: None,
            project_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn issue_type_id(mut self, value: String) -> Self {
        self.issue_type_id = Some(value);
        self
    }

    pub fn jira_account(
        mut self,
        value: crate::datadogV2::model::JiraIssueTemplateCreateRequestAttributesJiraAccount,
    ) -> Self {
        self.jira_account = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
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

impl Default for JiraIssueTemplateCreateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for JiraIssueTemplateCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueTemplateCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for JiraIssueTemplateCreateRequestAttributesVisitor {
            type Value = JiraIssueTemplateCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut issue_type_id: Option<String> = None;
                let mut jira_account: Option<
                    crate::datadogV2::model::JiraIssueTemplateCreateRequestAttributesJiraAccount,
                > = None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira-account" => {
                            if v.is_null() {
                                continue;
                            }
                            jira_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = JiraIssueTemplateCreateRequestAttributes {
                    fields,
                    issue_type_id,
                    jira_account,
                    name,
                    project_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueTemplateCreateRequestAttributesVisitor)
    }
}

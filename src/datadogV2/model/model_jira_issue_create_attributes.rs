// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Jira issue creation attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIssueCreateAttributes {
    /// Additional Jira fields
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Jira issue type ID
    #[serde(rename = "issue_type_id")]
    pub issue_type_id: String,
    /// Jira account ID
    #[serde(rename = "jira_account_id")]
    pub jira_account_id: String,
    /// Jira project ID
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueCreateAttributes {
    pub fn new(
        issue_type_id: String,
        jira_account_id: String,
        project_id: String,
    ) -> JiraIssueCreateAttributes {
        JiraIssueCreateAttributes {
            fields: None,
            issue_type_id,
            jira_account_id,
            project_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
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

impl<'de> Deserialize<'de> for JiraIssueCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueCreateAttributesVisitor;
        impl<'a> Visitor<'a> for JiraIssueCreateAttributesVisitor {
            type Value = JiraIssueCreateAttributes;

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
                let mut jira_account_id: Option<String> = None;
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
                            issue_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_account_id" => {
                            jira_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let issue_type_id =
                    issue_type_id.ok_or_else(|| M::Error::missing_field("issue_type_id"))?;
                let jira_account_id =
                    jira_account_id.ok_or_else(|| M::Error::missing_field("jira_account_id"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;

                let content = JiraIssueCreateAttributes {
                    fields,
                    issue_type_id,
                    jira_account_id,
                    project_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueCreateAttributesVisitor)
    }
}

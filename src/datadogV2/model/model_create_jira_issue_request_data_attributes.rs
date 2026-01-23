// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Jira issue to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateJiraIssueRequestDataAttributes {
    /// Unique identifier of the Datadog user assigned to the Jira issue.
    #[serde(rename = "assignee_id")]
    pub assignee_id: Option<String>,
    /// Description of the Jira issue. If not provided, the description will be automatically generated.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Custom fields of the Jira issue to create. For the list of available fields, see [Jira documentation](<https://developer.atlassian.com/cloud/jira/platform/rest/v2/api-group-issues/#api-rest-api-2-issue-createmeta-projectidorkey-issuetypes-issuetypeid-get>).
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Case priority
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::CasePriority>,
    /// Title of the Jira issue. If not provided, the title will be automatically generated.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateJiraIssueRequestDataAttributes {
    pub fn new() -> CreateJiraIssueRequestDataAttributes {
        CreateJiraIssueRequestDataAttributes {
            assignee_id: None,
            description: None,
            fields: None,
            priority: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee_id(mut self, value: String) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn fields(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn priority(mut self, value: crate::datadogV2::model::CasePriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for CreateJiraIssueRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateJiraIssueRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateJiraIssueRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateJiraIssueRequestDataAttributesVisitor {
            type Value = CreateJiraIssueRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee_id: Option<String> = None;
                let mut description: Option<String> = None;
                let mut fields: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut priority: Option<crate::datadogV2::model::CasePriority> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee_id" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateJiraIssueRequestDataAttributes {
                    assignee_id,
                    description,
                    fields,
                    priority,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateJiraIssueRequestDataAttributesVisitor)
    }
}

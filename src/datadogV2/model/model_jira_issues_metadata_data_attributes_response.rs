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
pub struct JiraIssuesMetadataDataAttributesResponse {
    /// Jira account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Jira issue type identifier.
    #[serde(rename = "issue_type_id")]
    pub issue_type_id: String,
    /// Jira project identifier.
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssuesMetadataDataAttributesResponse {
    pub fn new(
        account_id: String,
        issue_type_id: String,
        project_id: String,
    ) -> JiraIssuesMetadataDataAttributesResponse {
        JiraIssuesMetadataDataAttributesResponse {
            account_id,
            issue_type_id,
            project_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for JiraIssuesMetadataDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssuesMetadataDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for JiraIssuesMetadataDataAttributesResponseVisitor {
            type Value = JiraIssuesMetadataDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut issue_type_id: Option<String> = None;
                let mut project_id: Option<String> = None;
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
                        "issue_type_id" => {
                            issue_type_id =
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let issue_type_id =
                    issue_type_id.ok_or_else(|| M::Error::missing_field("issue_type_id"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;

                let content = JiraIssuesMetadataDataAttributesResponse {
                    account_id,
                    issue_type_id,
                    project_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssuesMetadataDataAttributesResponseVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an Agent Observability annotation queue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotationQueueDataAttributesRequest {
    /// Schema defining the labels for an annotation queue.
    #[serde(rename = "annotation_schema")]
    pub annotation_schema: Option<crate::datadogV2::model::LLMObsAnnotationSchema>,
    /// Description of the annotation queue.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the annotation queue.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the project this queue belongs to.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Whether annotation access is restricted to assigned users.
    #[serde(rename = "restrict_to_assignees")]
    pub restrict_to_assignees: Option<bool>,
    /// Whether annotation access is restricted to queue reviewers.
    #[serde(rename = "restrict_to_reviewers")]
    pub restrict_to_reviewers: Option<bool>,
    /// Email addresses of reviewers who can access the annotation queue.
    #[serde(rename = "reviewer_emails")]
    pub reviewer_emails: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotationQueueDataAttributesRequest {
    pub fn new(name: String, project_id: String) -> LLMObsAnnotationQueueDataAttributesRequest {
        LLMObsAnnotationQueueDataAttributesRequest {
            annotation_schema: None,
            description: None,
            name,
            project_id,
            restrict_to_assignees: None,
            restrict_to_reviewers: None,
            reviewer_emails: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn annotation_schema(
        mut self,
        value: crate::datadogV2::model::LLMObsAnnotationSchema,
    ) -> Self {
        self.annotation_schema = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn restrict_to_assignees(mut self, value: bool) -> Self {
        self.restrict_to_assignees = Some(value);
        self
    }

    pub fn restrict_to_reviewers(mut self, value: bool) -> Self {
        self.restrict_to_reviewers = Some(value);
        self
    }

    pub fn reviewer_emails(mut self, value: Vec<String>) -> Self {
        self.reviewer_emails = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotationQueueDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotationQueueDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotationQueueDataAttributesRequestVisitor {
            type Value = LLMObsAnnotationQueueDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_schema: Option<crate::datadogV2::model::LLMObsAnnotationSchema> =
                    None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut restrict_to_assignees: Option<bool> = None;
                let mut restrict_to_reviewers: Option<bool> = None;
                let mut reviewer_emails: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_schema" => {
                            if v.is_null() {
                                continue;
                            }
                            annotation_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restrict_to_assignees" => {
                            if v.is_null() {
                                continue;
                            }
                            restrict_to_assignees =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restrict_to_reviewers" => {
                            if v.is_null() {
                                continue;
                            }
                            restrict_to_reviewers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reviewer_emails" => {
                            if v.is_null() {
                                continue;
                            }
                            reviewer_emails =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;

                let content = LLMObsAnnotationQueueDataAttributesRequest {
                    annotation_schema,
                    description,
                    name,
                    project_id,
                    restrict_to_assignees,
                    restrict_to_reviewers,
                    reviewer_emails,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotationQueueDataAttributesRequestVisitor)
    }
}

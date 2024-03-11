// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident todo's attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTodoAttributes {
    /// Array of todo assignees.
    #[serde(rename = "assignees")]
    pub assignees: Vec<crate::datadogV2::model::IncidentTodoAssignee>,
    /// Timestamp when the todo was completed.
    #[serde(
        rename = "completed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub completed: Option<Option<String>>,
    /// The follow-up task's content.
    #[serde(rename = "content")]
    pub content: String,
    /// Timestamp when the incident todo was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Timestamp when the todo should be completed by.
    #[serde(
        rename = "due_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub due_date: Option<Option<String>>,
    /// UUID of the incident this todo is connected to.
    #[serde(rename = "incident_id")]
    pub incident_id: Option<String>,
    /// Timestamp when the incident todo was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTodoAttributes {
    pub fn new(
        assignees: Vec<crate::datadogV2::model::IncidentTodoAssignee>,
        content: String,
    ) -> IncidentTodoAttributes {
        IncidentTodoAttributes {
            assignees,
            completed: None,
            content,
            created: None,
            due_date: None,
            incident_id: None,
            modified: None,
            _unparsed: false,
        }
    }

    pub fn completed(&mut self, value: Option<String>) -> &mut Self {
        self.completed = Some(value);
        self
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn due_date(&mut self, value: Option<String>) -> &mut Self {
        self.due_date = Some(value);
        self
    }

    pub fn incident_id(&mut self, value: String) -> &mut Self {
        self.incident_id = Some(value);
        self
    }

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentTodoAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTodoAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentTodoAttributesVisitor {
            type Value = IncidentTodoAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignees: Option<Vec<crate::datadogV2::model::IncidentTodoAssignee>> =
                    None;
                let mut completed: Option<Option<String>> = None;
                let mut content: Option<String> = None;
                let mut created: Option<String> = None;
                let mut due_date: Option<Option<String>> = None;
                let mut incident_id: Option<String> = None;
                let mut modified: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignees" => {
                            assignees = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "completed" => {
                            completed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "due_date" => {
                            due_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let assignees = assignees.ok_or_else(|| M::Error::missing_field("assignees"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;

                let content = IncidentTodoAttributes {
                    assignees,
                    completed,
                    content,
                    created,
                    due_date,
                    incident_id,
                    modified,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTodoAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a change request response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestResponseAttributes {
    /// Timestamp of when the change request was archived.
    #[serde(
        rename = "archived_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Custom attributes of the change request as key-value pairs.
    #[serde(rename = "attributes")]
    pub attributes: std::collections::BTreeMap<String, Vec<String>>,
    /// The UUID of the linked incident.
    #[serde(rename = "change_request_linked_incident_uuid")]
    pub change_request_linked_incident_uuid: String,
    /// The maintenance window query for the change request.
    #[serde(rename = "change_request_maintenance_window_query")]
    pub change_request_maintenance_window_query: String,
    /// The plan associated with the change request.
    #[serde(rename = "change_request_plan")]
    pub change_request_plan: String,
    /// The risk level of the change request.
    #[serde(rename = "change_request_risk")]
    pub change_request_risk: crate::datadogV2::model::ChangeRequestRiskLevel,
    /// The type of the change request.
    #[serde(rename = "change_request_type")]
    pub change_request_type: crate::datadogV2::model::ChangeRequestChangeType,
    /// Timestamp of when the change request was closed.
    #[serde(
        rename = "closed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub closed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Timestamp of when the change request was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The source from which the change request was created.
    #[serde(rename = "creation_source")]
    pub creation_source: String,
    /// The description of the change request.
    #[serde(rename = "description")]
    pub description: String,
    /// The planned end date of the change request.
    #[serde(rename = "end_date")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The human-readable key of the change request.
    #[serde(rename = "key")]
    pub key: String,
    /// Timestamp of when the change request was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The notebook ID associated with the change request plan.
    #[serde(rename = "plan_notebook_id")]
    pub plan_notebook_id: i64,
    /// The priority of the change request.
    #[serde(rename = "priority")]
    pub priority: String,
    /// The project UUID associated with the change request.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The planned start date of the change request.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The current status of the change request.
    #[serde(rename = "status")]
    pub status: String,
    /// The title of the change request.
    #[serde(rename = "title")]
    pub title: String,
    /// The case type.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestResponseAttributes {
    pub fn new(
        attributes: std::collections::BTreeMap<String, Vec<String>>,
        change_request_linked_incident_uuid: String,
        change_request_maintenance_window_query: String,
        change_request_plan: String,
        change_request_risk: crate::datadogV2::model::ChangeRequestRiskLevel,
        change_request_type: crate::datadogV2::model::ChangeRequestChangeType,
        created_at: chrono::DateTime<chrono::Utc>,
        creation_source: String,
        description: String,
        key: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        plan_notebook_id: i64,
        priority: String,
        project_id: String,
        status: String,
        title: String,
        type_: String,
    ) -> ChangeRequestResponseAttributes {
        ChangeRequestResponseAttributes {
            archived_at: None,
            attributes,
            change_request_linked_incident_uuid,
            change_request_maintenance_window_query,
            change_request_plan,
            change_request_risk,
            change_request_type,
            closed_at: None,
            created_at,
            creation_source,
            description,
            end_date: None,
            key,
            modified_at,
            plan_notebook_id,
            priority,
            project_id,
            start_date: None,
            status,
            title,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn closed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(value);
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

impl<'de> Deserialize<'de> for ChangeRequestResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeRequestResponseAttributesVisitor {
            type Value = ChangeRequestResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut attributes: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut change_request_linked_incident_uuid: Option<String> = None;
                let mut change_request_maintenance_window_query: Option<String> = None;
                let mut change_request_plan: Option<String> = None;
                let mut change_request_risk: Option<
                    crate::datadogV2::model::ChangeRequestRiskLevel,
                > = None;
                let mut change_request_type: Option<
                    crate::datadogV2::model::ChangeRequestChangeType,
                > = None;
                let mut closed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creation_source: Option<String> = None;
                let mut description: Option<String> = None;
                let mut end_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut key: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut plan_notebook_id: Option<i64> = None;
                let mut priority: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<String> = None;
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
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_request_linked_incident_uuid" => {
                            change_request_linked_incident_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_request_maintenance_window_query" => {
                            change_request_maintenance_window_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_request_plan" => {
                            change_request_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_request_risk" => {
                            change_request_risk =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_risk) = change_request_risk {
                                match _change_request_risk {
                                    crate::datadogV2::model::ChangeRequestRiskLevel::UnparsedObject(_change_request_risk) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "change_request_type" => {
                            change_request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_type) = change_request_type {
                                match _change_request_type {
                                    crate::datadogV2::model::ChangeRequestChangeType::UnparsedObject(_change_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "closed_at" => {
                            closed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_source" => {
                            creation_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "plan_notebook_id" => {
                            plan_notebook_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let change_request_linked_incident_uuid = change_request_linked_incident_uuid
                    .ok_or_else(|| {
                        M::Error::missing_field("change_request_linked_incident_uuid")
                    })?;
                let change_request_maintenance_window_query =
                    change_request_maintenance_window_query.ok_or_else(|| {
                        M::Error::missing_field("change_request_maintenance_window_query")
                    })?;
                let change_request_plan = change_request_plan
                    .ok_or_else(|| M::Error::missing_field("change_request_plan"))?;
                let change_request_risk = change_request_risk
                    .ok_or_else(|| M::Error::missing_field("change_request_risk"))?;
                let change_request_type = change_request_type
                    .ok_or_else(|| M::Error::missing_field("change_request_type"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let creation_source =
                    creation_source.ok_or_else(|| M::Error::missing_field("creation_source"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let plan_notebook_id =
                    plan_notebook_id.ok_or_else(|| M::Error::missing_field("plan_notebook_id"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ChangeRequestResponseAttributes {
                    archived_at,
                    attributes,
                    change_request_linked_incident_uuid,
                    change_request_maintenance_window_query,
                    change_request_plan,
                    change_request_risk,
                    change_request_type,
                    closed_at,
                    created_at,
                    creation_source,
                    description,
                    end_date,
                    key,
                    modified_at,
                    plan_notebook_id,
                    priority,
                    project_id,
                    start_date,
                    status,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestResponseAttributesVisitor)
    }
}

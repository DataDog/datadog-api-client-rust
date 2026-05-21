// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident Jira template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentJiraTemplateDataAttributesResponse {
    /// The Jira account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Timestamp when the template was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// UUID of the user who created the template.
    #[serde(rename = "created_by_uuid")]
    pub created_by_uuid: uuid::Uuid,
    /// Field configuration mappings.
    #[serde(rename = "field_configurations")]
    pub field_configurations:
        Option<Vec<crate::datadogV2::model::IncidentJiraTemplateFieldConfiguration>>,
    /// The Jira fields configuration.
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether this is the default template.
    #[serde(rename = "is_default")]
    pub is_default: bool,
    /// The Jira issue type identifier.
    #[serde(rename = "issue_id")]
    pub issue_id: String,
    /// UUID of the user who last modified the template.
    #[serde(rename = "last_modified_by_uuid")]
    pub last_modified_by_uuid: uuid::Uuid,
    /// The field mappings configuration.
    #[serde(rename = "mappings")]
    pub mappings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Additional metadata for the template.
    #[serde(rename = "meta")]
    pub meta: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Timestamp when the template was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// The name of the template.
    #[serde(rename = "name")]
    pub name: String,
    /// The Jira project identifier.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The Jira project key.
    #[serde(rename = "project_key")]
    pub project_key: String,
    /// Whether synchronization is enabled.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: bool,
    /// The type of the template.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentJiraTemplateDataAttributesResponse {
    pub fn new(
        account_id: String,
        created: chrono::DateTime<chrono::Utc>,
        created_by_uuid: uuid::Uuid,
        is_default: bool,
        issue_id: String,
        last_modified_by_uuid: uuid::Uuid,
        modified: chrono::DateTime<chrono::Utc>,
        name: String,
        project_id: String,
        project_key: String,
        sync_enabled: bool,
        type_: String,
    ) -> IncidentJiraTemplateDataAttributesResponse {
        IncidentJiraTemplateDataAttributesResponse {
            account_id,
            created,
            created_by_uuid,
            field_configurations: None,
            fields: None,
            is_default,
            issue_id,
            last_modified_by_uuid,
            mappings: None,
            meta: None,
            modified,
            name,
            project_id,
            project_key,
            sync_enabled,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn field_configurations(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentJiraTemplateFieldConfiguration>,
    ) -> Self {
        self.field_configurations = Some(value);
        self
    }

    pub fn fields(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn mappings(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.mappings = Some(value);
        self
    }

    pub fn meta(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for IncidentJiraTemplateDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentJiraTemplateDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentJiraTemplateDataAttributesResponseVisitor {
            type Value = IncidentJiraTemplateDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by_uuid: Option<uuid::Uuid> = None;
                let mut field_configurations: Option<
                    Vec<crate::datadogV2::model::IncidentJiraTemplateFieldConfiguration>,
                > = None;
                let mut fields: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut is_default: Option<bool> = None;
                let mut issue_id: Option<String> = None;
                let mut last_modified_by_uuid: Option<uuid::Uuid> = None;
                let mut mappings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut meta: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut project_key: Option<String> = None;
                let mut sync_enabled: Option<bool> = None;
                let mut type_: Option<String> = None;
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
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_uuid" => {
                            created_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_configurations" => {
                            if v.is_null() {
                                continue;
                            }
                            field_configurations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_id" => {
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_uuid" => {
                            last_modified_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mappings" => {
                            if v.is_null() {
                                continue;
                            }
                            mappings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_key" => {
                            project_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_enabled" => {
                            sync_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let created_by_uuid =
                    created_by_uuid.ok_or_else(|| M::Error::missing_field("created_by_uuid"))?;
                let is_default = is_default.ok_or_else(|| M::Error::missing_field("is_default"))?;
                let issue_id = issue_id.ok_or_else(|| M::Error::missing_field("issue_id"))?;
                let last_modified_by_uuid = last_modified_by_uuid
                    .ok_or_else(|| M::Error::missing_field("last_modified_by_uuid"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let project_key =
                    project_key.ok_or_else(|| M::Error::missing_field("project_key"))?;
                let sync_enabled =
                    sync_enabled.ok_or_else(|| M::Error::missing_field("sync_enabled"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = IncidentJiraTemplateDataAttributesResponse {
                    account_id,
                    created,
                    created_by_uuid,
                    field_configurations,
                    fields,
                    is_default,
                    issue_id,
                    last_modified_by_uuid,
                    mappings,
                    meta,
                    modified,
                    name,
                    project_id,
                    project_key,
                    sync_enabled,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentJiraTemplateDataAttributesResponseVisitor)
    }
}

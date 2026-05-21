// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating an incident Jira template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentJiraTemplateDataAttributesRequest {
    /// The Jira account identifier.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Field configuration mappings.
    #[serde(rename = "field_configurations")]
    pub field_configurations:
        Option<Vec<crate::datadogV2::model::IncidentJiraTemplateFieldConfiguration>>,
    /// The Jira fields configuration.
    #[serde(rename = "fields")]
    pub fields: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether this is the default template.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// The Jira issue type identifier.
    #[serde(rename = "issue_id")]
    pub issue_id: String,
    /// The field mappings configuration.
    #[serde(rename = "mappings")]
    pub mappings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Additional metadata for the template.
    #[serde(rename = "meta")]
    pub meta: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The name of the template.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The Jira project identifier.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The Jira project key.
    #[serde(rename = "project_key")]
    pub project_key: String,
    /// Whether synchronization is enabled.
    #[serde(rename = "sync_enabled")]
    pub sync_enabled: Option<bool>,
    /// The type of the template.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentJiraTemplateDataAttributesRequest {
    pub fn new(
        account_id: String,
        issue_id: String,
        project_id: String,
        project_key: String,
    ) -> IncidentJiraTemplateDataAttributesRequest {
        IncidentJiraTemplateDataAttributesRequest {
            account_id,
            field_configurations: None,
            fields: None,
            is_default: None,
            issue_id,
            mappings: None,
            meta: None,
            name: None,
            project_id,
            project_key,
            sync_enabled: None,
            type_: None,
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

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
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

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for IncidentJiraTemplateDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentJiraTemplateDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentJiraTemplateDataAttributesRequestVisitor {
            type Value = IncidentJiraTemplateDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut field_configurations: Option<
                    Vec<crate::datadogV2::model::IncidentJiraTemplateFieldConfiguration>,
                > = None;
                let mut fields: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut is_default: Option<bool> = None;
                let mut issue_id: Option<String> = None;
                let mut mappings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut meta: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_id" => {
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            sync_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
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
                let issue_id = issue_id.ok_or_else(|| M::Error::missing_field("issue_id"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let project_key =
                    project_key.ok_or_else(|| M::Error::missing_field("project_key"))?;

                let content = IncidentJiraTemplateDataAttributesRequest {
                    account_id,
                    field_configurations,
                    fields,
                    is_default,
                    issue_id,
                    mappings,
                    meta,
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

        deserializer.deserialize_any(IncidentJiraTemplateDataAttributesRequestVisitor)
    }
}

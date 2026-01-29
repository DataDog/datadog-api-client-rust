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
pub struct IntegrationJiraSyncProperties {
    /// Sync property configuration
    #[serde(rename = "assignee")]
    pub assignee: Option<crate::datadogV2::model::SyncProperty>,
    /// Sync property configuration
    #[serde(rename = "comments")]
    pub comments: Option<crate::datadogV2::model::SyncProperty>,
    #[serde(rename = "custom_fields")]
    pub custom_fields: Option<
        std::collections::BTreeMap<
            String,
            crate::datadogV2::model::IntegrationJiraSyncPropertiesCustomFieldsAdditionalProperties,
        >,
    >,
    /// Sync property configuration
    #[serde(rename = "description")]
    pub description: Option<crate::datadogV2::model::SyncProperty>,
    #[serde(rename = "due_date")]
    pub due_date: Option<crate::datadogV2::model::IntegrationJiraSyncDueDate>,
    /// Sync property with mapping configuration
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::SyncPropertyWithMapping>,
    /// Sync property with mapping configuration
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SyncPropertyWithMapping>,
    /// Sync property configuration
    #[serde(rename = "title")]
    pub title: Option<crate::datadogV2::model::SyncProperty>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationJiraSyncProperties {
    pub fn new() -> IntegrationJiraSyncProperties {
        IntegrationJiraSyncProperties {
            assignee: None,
            comments: None,
            custom_fields: None,
            description: None,
            due_date: None,
            priority: None,
            status: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee(mut self, value: crate::datadogV2::model::SyncProperty) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn comments(mut self, value: crate::datadogV2::model::SyncProperty) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn custom_fields(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::IntegrationJiraSyncPropertiesCustomFieldsAdditionalProperties,
        >,
    ) -> Self {
        self.custom_fields = Some(value);
        self
    }

    pub fn description(mut self, value: crate::datadogV2::model::SyncProperty) -> Self {
        self.description = Some(value);
        self
    }

    pub fn due_date(mut self, value: crate::datadogV2::model::IntegrationJiraSyncDueDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn priority(mut self, value: crate::datadogV2::model::SyncPropertyWithMapping) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::SyncPropertyWithMapping) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: crate::datadogV2::model::SyncProperty) -> Self {
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

impl Default for IntegrationJiraSyncProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationJiraSyncProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationJiraSyncPropertiesVisitor;
        impl<'a> Visitor<'a> for IntegrationJiraSyncPropertiesVisitor {
            type Value = IntegrationJiraSyncProperties;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee: Option<crate::datadogV2::model::SyncProperty> = None;
                let mut comments: Option<crate::datadogV2::model::SyncProperty> = None;
                let mut custom_fields: Option<std::collections::BTreeMap<String, crate::datadogV2::model::IntegrationJiraSyncPropertiesCustomFieldsAdditionalProperties>> = None;
                let mut description: Option<crate::datadogV2::model::SyncProperty> = None;
                let mut due_date: Option<crate::datadogV2::model::IntegrationJiraSyncDueDate> =
                    None;
                let mut priority: Option<crate::datadogV2::model::SyncPropertyWithMapping> = None;
                let mut status: Option<crate::datadogV2::model::SyncPropertyWithMapping> = None;
                let mut title: Option<crate::datadogV2::model::SyncProperty> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comments" => {
                            if v.is_null() {
                                continue;
                            }
                            comments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "due_date" => {
                            if v.is_null() {
                                continue;
                            }
                            due_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IntegrationJiraSyncProperties {
                    assignee,
                    comments,
                    custom_fields,
                    description,
                    due_date,
                    priority,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationJiraSyncPropertiesVisitor)
    }
}

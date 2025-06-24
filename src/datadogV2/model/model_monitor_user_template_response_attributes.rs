// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a monitor user template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorUserTemplateResponseAttributes {
    /// The created timestamp of the template.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// A brief description of the monitor user template.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The last modified timestamp. When the template version was created.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// A valid monitor definition in the same format as the [V1 Monitor API](<https://docs.datadoghq.com/api/latest/monitors/#create-a-monitor>).
    #[serde(rename = "monitor_definition")]
    pub monitor_definition: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The definition of `MonitorUserTemplateTags` object.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The definition of `MonitorUserTemplateTemplateVariables` object.
    #[serde(rename = "template_variables")]
    pub template_variables:
        Option<Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>>,
    /// The title of the monitor user template.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The version of the monitor user template.
    #[serde(
        rename = "version",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub version: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorUserTemplateResponseAttributes {
    pub fn new() -> MonitorUserTemplateResponseAttributes {
        MonitorUserTemplateResponseAttributes {
            created: None,
            description: None,
            modified: None,
            monitor_definition: None,
            tags: None,
            template_variables: None,
            title: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn monitor_definition(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.monitor_definition = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn version(mut self, value: Option<i64>) -> Self {
        self.version = Some(value);
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

impl Default for MonitorUserTemplateResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorUserTemplateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorUserTemplateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorUserTemplateResponseAttributesVisitor {
            type Value = MonitorUserTemplateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut monitor_definition: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>,
                > = None;
                let mut title: Option<String> = None;
                let mut version: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorUserTemplateResponseAttributes {
                    created,
                    description,
                    modified,
                    monitor_definition,
                    tags,
                    template_variables,
                    title,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorUserTemplateResponseAttributesVisitor)
    }
}

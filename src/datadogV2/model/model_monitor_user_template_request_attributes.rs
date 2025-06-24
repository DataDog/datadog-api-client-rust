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
pub struct MonitorUserTemplateRequestAttributes {
    /// A brief description of the monitor user template.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// A valid monitor definition in the same format as the [V1 Monitor API](<https://docs.datadoghq.com/api/latest/monitors/#create-a-monitor>).
    #[serde(rename = "monitor_definition")]
    pub monitor_definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// The definition of `MonitorUserTemplateTags` object.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The definition of `MonitorUserTemplateTemplateVariables` object.
    #[serde(rename = "template_variables")]
    pub template_variables:
        Option<Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>>,
    /// The title of the monitor user template.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorUserTemplateRequestAttributes {
    pub fn new(
        monitor_definition: std::collections::BTreeMap<String, serde_json::Value>,
        tags: Vec<String>,
        title: String,
    ) -> MonitorUserTemplateRequestAttributes {
        MonitorUserTemplateRequestAttributes {
            description: None,
            monitor_definition,
            tags,
            template_variables: None,
            title,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorUserTemplateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorUserTemplateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorUserTemplateRequestAttributesVisitor {
            type Value = MonitorUserTemplateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut monitor_definition: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::MonitorUserTemplateTemplateVariablesItems>,
                > = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_definition" => {
                            monitor_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
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
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let monitor_definition = monitor_definition
                    .ok_or_else(|| M::Error::missing_field("monitor_definition"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = MonitorUserTemplateRequestAttributes {
                    description,
                    monitor_definition,
                    tags,
                    template_variables,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorUserTemplateRequestAttributesVisitor)
    }
}

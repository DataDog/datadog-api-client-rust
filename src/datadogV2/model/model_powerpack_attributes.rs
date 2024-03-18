// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack attribute object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackAttributes {
    /// Description of this powerpack.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Powerpack group widget definition object.
    #[serde(rename = "group_widget")]
    pub group_widget: crate::datadogV2::model::PowerpackGroupWidget,
    /// Name of the powerpack.
    #[serde(rename = "name")]
    pub name: String,
    /// List of tags to identify this powerpack.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// List of template variables for this powerpack.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<Vec<crate::datadogV2::model::PowerpackTemplateVariable>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackAttributes {
    pub fn new(
        group_widget: crate::datadogV2::model::PowerpackGroupWidget,
        name: String,
    ) -> PowerpackAttributes {
        PowerpackAttributes {
            description: None,
            group_widget,
            name,
            tags: None,
            template_variables: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV2::model::PowerpackTemplateVariable>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackAttributesVisitor;
        impl<'a> Visitor<'a> for PowerpackAttributesVisitor {
            type Value = PowerpackAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut group_widget: Option<crate::datadogV2::model::PowerpackGroupWidget> = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::PowerpackTemplateVariable>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_widget" => {
                            group_widget =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }
                let group_widget =
                    group_widget.ok_or_else(|| M::Error::missing_field("group_widget"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = PowerpackAttributes {
                    description,
                    group_widget,
                    name,
                    tags,
                    template_variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackAttributesVisitor)
    }
}

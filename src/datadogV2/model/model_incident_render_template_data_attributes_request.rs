// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for rendering a template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRenderTemplateDataAttributesRequest {
    /// The template content to render.
    #[serde(rename = "content")]
    pub content: String,
    /// The date-time format to use for rendering.
    #[serde(rename = "datetime_format")]
    pub datetime_format: Option<String>,
    /// The timezone to use for rendering.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Whether to validate links in the rendered template.
    #[serde(
        rename = "validate_links",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub validate_links: Option<Option<bool>>,
    /// Whether to validate variables in the template.
    #[serde(
        rename = "validate_variables",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub validate_variables: Option<Option<bool>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRenderTemplateDataAttributesRequest {
    pub fn new(content: String) -> IncidentRenderTemplateDataAttributesRequest {
        IncidentRenderTemplateDataAttributesRequest {
            content,
            datetime_format: None,
            timezone: None,
            validate_links: None,
            validate_variables: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn datetime_format(mut self, value: String) -> Self {
        self.datetime_format = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn validate_links(mut self, value: Option<bool>) -> Self {
        self.validate_links = Some(value);
        self
    }

    pub fn validate_variables(mut self, value: Option<bool>) -> Self {
        self.validate_variables = Some(value);
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

impl<'de> Deserialize<'de> for IncidentRenderTemplateDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRenderTemplateDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentRenderTemplateDataAttributesRequestVisitor {
            type Value = IncidentRenderTemplateDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut datetime_format: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut validate_links: Option<Option<bool>> = None;
                let mut validate_variables: Option<Option<bool>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datetime_format" => {
                            if v.is_null() {
                                continue;
                            }
                            datetime_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validate_links" => {
                            validate_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validate_variables" => {
                            validate_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;

                let content = IncidentRenderTemplateDataAttributesRequest {
                    content,
                    datetime_format,
                    timezone,
                    validate_links,
                    validate_variables,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRenderTemplateDataAttributesRequestVisitor)
    }
}

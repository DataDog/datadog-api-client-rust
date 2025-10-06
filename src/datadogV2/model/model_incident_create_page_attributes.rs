// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a page from an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCreatePageAttributes {
    /// Description of the page.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// List of services associated with the page.
    #[serde(
        rename = "services",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub services: Option<Option<Vec<String>>>,
    /// List of tags for the page.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Target for creating a page from an incident.
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::IncidentPageTarget,
    /// Title of the page.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCreatePageAttributes {
    pub fn new(
        target: crate::datadogV2::model::IncidentPageTarget,
        title: String,
    ) -> IncidentCreatePageAttributes {
        IncidentCreatePageAttributes {
            description: None,
            services: None,
            tags: None,
            target,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn services(mut self, value: Option<Vec<String>>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for IncidentCreatePageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCreatePageAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentCreatePageAttributesVisitor {
            type Value = IncidentCreatePageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut services: Option<Option<Vec<String>>> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut target: Option<crate::datadogV2::model::IncidentPageTarget> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentCreatePageAttributes {
                    description,
                    services,
                    tags,
                    target,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCreatePageAttributesVisitor)
    }
}

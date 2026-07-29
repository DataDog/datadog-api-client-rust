// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an on-call page from an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCreateOnCallPageDataAttributesRequest {
    /// The description of the page.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A reference to an incident role for a page.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV2::model::IncidentPageRoleReference>,
    /// List of affected services.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// List of tags for the page.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The target recipient for a page.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::IncidentPageTarget>,
    /// The title of the page.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCreateOnCallPageDataAttributesRequest {
    pub fn new() -> IncidentCreateOnCallPageDataAttributesRequest {
        IncidentCreateOnCallPageDataAttributesRequest {
            description: None,
            role: None,
            services: None,
            tags: None,
            target: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn role(mut self, value: crate::datadogV2::model::IncidentPageRoleReference) -> Self {
        self.role = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn target(mut self, value: crate::datadogV2::model::IncidentPageTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
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

impl Default for IncidentCreateOnCallPageDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentCreateOnCallPageDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCreateOnCallPageDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentCreateOnCallPageDataAttributesRequestVisitor {
            type Value = IncidentCreateOnCallPageDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut role: Option<crate::datadogV2::model::IncidentPageRoleReference> = None;
                let mut services: Option<Vec<String>> = None;
                let mut tags: Option<Vec<String>> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentCreateOnCallPageDataAttributesRequest {
                    description,
                    role,
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

        deserializer.deserialize_any(IncidentCreateOnCallPageDataAttributesRequestVisitor)
    }
}

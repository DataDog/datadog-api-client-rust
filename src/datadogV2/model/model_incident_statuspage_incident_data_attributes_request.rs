// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a Statuspage incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentStatuspageIncidentDataAttributesRequest {
    /// The body text of the Statuspage incident.
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option")]
    pub body: Option<Option<String>>,
    /// Map of component identifiers to their status.
    #[serde(rename = "components")]
    pub components: Option<std::collections::BTreeMap<String, String>>,
    /// Whether to deliver notifications.
    #[serde(
        rename = "deliver_notifications",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deliver_notifications: Option<Option<bool>>,
    /// The impact level of the incident.
    #[serde(rename = "impact", default, with = "::serde_with::rust::double_option")]
    pub impact: Option<Option<String>>,
    /// The name of the Statuspage incident.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// The Statuspage page identifier.
    #[serde(rename = "page_id")]
    pub page_id: Option<String>,
    /// The status of the Statuspage incident.
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option")]
    pub status: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentStatuspageIncidentDataAttributesRequest {
    pub fn new() -> IncidentStatuspageIncidentDataAttributesRequest {
        IncidentStatuspageIncidentDataAttributesRequest {
            body: None,
            components: None,
            deliver_notifications: None,
            impact: None,
            name: None,
            page_id: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn body(mut self, value: Option<String>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn components(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn deliver_notifications(mut self, value: Option<bool>) -> Self {
        self.deliver_notifications = Some(value);
        self
    }

    pub fn impact(mut self, value: Option<String>) -> Self {
        self.impact = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn page_id(mut self, value: String) -> Self {
        self.page_id = Some(value);
        self
    }

    pub fn status(mut self, value: Option<String>) -> Self {
        self.status = Some(value);
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

impl Default for IncidentStatuspageIncidentDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentStatuspageIncidentDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentStatuspageIncidentDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentStatuspageIncidentDataAttributesRequestVisitor {
            type Value = IncidentStatuspageIncidentDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut body: Option<Option<String>> = None;
                let mut components: Option<std::collections::BTreeMap<String, String>> = None;
                let mut deliver_notifications: Option<Option<bool>> = None;
                let mut impact: Option<Option<String>> = None;
                let mut name: Option<Option<String>> = None;
                let mut page_id: Option<String> = None;
                let mut status: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "body" => {
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deliver_notifications" => {
                            deliver_notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact" => {
                            impact = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_id" => {
                            if v.is_null() {
                                continue;
                            }
                            page_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentStatuspageIncidentDataAttributesRequest {
                    body,
                    components,
                    deliver_notifications,
                    impact,
                    name,
                    page_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentStatuspageIncidentDataAttributesRequestVisitor)
    }
}

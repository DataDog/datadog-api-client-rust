// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Salesforce incident template attributes returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SalesforceIncidentsTemplateResponseAttributes {
    /// Long-form description body for Salesforce incidents created from this template.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Human-readable name for this incident template.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The Salesforce user ID that owns incidents created from this template.
    #[serde(rename = "owner_id")]
    pub owner_id: Option<String>,
    /// Priority of the Salesforce incident created from this template.
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::SalesforceIncidentsTemplatePriority>,
    /// The Datadog-assigned ID of the Salesforce organization this template belongs to.
    #[serde(rename = "salesforce_org_id")]
    pub salesforce_org_id: Option<uuid::Uuid>,
    /// Subject line for Salesforce incidents created from this template.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SalesforceIncidentsTemplateResponseAttributes {
    pub fn new() -> SalesforceIncidentsTemplateResponseAttributes {
        SalesforceIncidentsTemplateResponseAttributes {
            description: None,
            name: None,
            owner_id: None,
            priority: None,
            salesforce_org_id: None,
            subject: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn owner_id(mut self, value: String) -> Self {
        self.owner_id = Some(value);
        self
    }

    pub fn priority(
        mut self,
        value: crate::datadogV2::model::SalesforceIncidentsTemplatePriority,
    ) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn salesforce_org_id(mut self, value: uuid::Uuid) -> Self {
        self.salesforce_org_id = Some(value);
        self
    }

    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
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

impl Default for SalesforceIncidentsTemplateResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SalesforceIncidentsTemplateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SalesforceIncidentsTemplateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SalesforceIncidentsTemplateResponseAttributesVisitor {
            type Value = SalesforceIncidentsTemplateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut owner_id: Option<String> = None;
                let mut priority: Option<
                    crate::datadogV2::model::SalesforceIncidentsTemplatePriority,
                > = None;
                let mut salesforce_org_id: Option<uuid::Uuid> = None;
                let mut subject: Option<String> = None;
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner_id" => {
                            if v.is_null() {
                                continue;
                            }
                            owner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::SalesforceIncidentsTemplatePriority::UnparsedObject(_priority) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "salesforce_org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            salesforce_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SalesforceIncidentsTemplateResponseAttributes {
                    description,
                    name,
                    owner_id,
                    priority,
                    salesforce_org_id,
                    subject,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SalesforceIncidentsTemplateResponseAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Salesforce incident template attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SalesforceIncidentsTemplateCreateAttributes {
    /// Long-form description body for Salesforce incidents created from this template.
    #[serde(rename = "description")]
    pub description: String,
    /// Human-readable name for this incident template. Must be unique within your organization.
    #[serde(rename = "name")]
    pub name: String,
    /// The Salesforce user ID that owns incidents created from this template.
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    /// Priority of the Salesforce incident created from this template.
    #[serde(rename = "priority")]
    pub priority: crate::datadogV2::model::SalesforceIncidentsTemplatePriority,
    /// The Datadog-assigned ID of the Salesforce organization this template belongs to.
    #[serde(rename = "salesforce_org_id")]
    pub salesforce_org_id: uuid::Uuid,
    /// Subject line for Salesforce incidents created from this template.
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SalesforceIncidentsTemplateCreateAttributes {
    pub fn new(
        description: String,
        name: String,
        owner_id: String,
        priority: crate::datadogV2::model::SalesforceIncidentsTemplatePriority,
        salesforce_org_id: uuid::Uuid,
        subject: String,
    ) -> SalesforceIncidentsTemplateCreateAttributes {
        SalesforceIncidentsTemplateCreateAttributes {
            description,
            name,
            owner_id,
            priority,
            salesforce_org_id,
            subject,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SalesforceIncidentsTemplateCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SalesforceIncidentsTemplateCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SalesforceIncidentsTemplateCreateAttributesVisitor {
            type Value = SalesforceIncidentsTemplateCreateAttributes;

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
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner_id" => {
                            owner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
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
                            salesforce_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let owner_id = owner_id.ok_or_else(|| M::Error::missing_field("owner_id"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let salesforce_org_id = salesforce_org_id
                    .ok_or_else(|| M::Error::missing_field("salesforce_org_id"))?;
                let subject = subject.ok_or_else(|| M::Error::missing_field("subject"))?;

                let content = SalesforceIncidentsTemplateCreateAttributes {
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

        deserializer.deserialize_any(SalesforceIncidentsTemplateCreateAttributesVisitor)
    }
}

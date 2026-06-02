// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Salesforce organization connected to the Datadog Salesforce integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SalesforceIncidentsOrganizationResponseAttributes {
    /// The Salesforce instance URL used to call this organization's APIs.
    #[serde(rename = "instance_url")]
    pub instance_url: Option<String>,
    /// Human-readable name of the Salesforce organization.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The Salesforce organization identifier (15- or 18-character Salesforce org ID).
    #[serde(rename = "sfdc_org_id")]
    pub sfdc_org_id: Option<String>,
    /// The Salesforce organization type (for example, `Production` or `Sandbox`).
    #[serde(rename = "sfdc_org_type")]
    pub sfdc_org_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SalesforceIncidentsOrganizationResponseAttributes {
    pub fn new() -> SalesforceIncidentsOrganizationResponseAttributes {
        SalesforceIncidentsOrganizationResponseAttributes {
            instance_url: None,
            name: None,
            sfdc_org_id: None,
            sfdc_org_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn instance_url(mut self, value: String) -> Self {
        self.instance_url = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn sfdc_org_id(mut self, value: String) -> Self {
        self.sfdc_org_id = Some(value);
        self
    }

    pub fn sfdc_org_type(mut self, value: String) -> Self {
        self.sfdc_org_type = Some(value);
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

impl Default for SalesforceIncidentsOrganizationResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SalesforceIncidentsOrganizationResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SalesforceIncidentsOrganizationResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SalesforceIncidentsOrganizationResponseAttributesVisitor {
            type Value = SalesforceIncidentsOrganizationResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut instance_url: Option<String> = None;
                let mut name: Option<String> = None;
                let mut sfdc_org_id: Option<String> = None;
                let mut sfdc_org_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "instance_url" => {
                            if v.is_null() {
                                continue;
                            }
                            instance_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sfdc_org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            sfdc_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sfdc_org_type" => {
                            if v.is_null() {
                                continue;
                            }
                            sfdc_org_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SalesforceIncidentsOrganizationResponseAttributes {
                    instance_url,
                    name,
                    sfdc_org_id,
                    sfdc_org_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SalesforceIncidentsOrganizationResponseAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a Jira field mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentJiraTemplateFieldConfiguration {
    /// Custom value for outbound synchronization.
    #[serde(rename = "custom_outbound_value")]
    pub custom_outbound_value: Option<serde_json::Value>,
    /// The incident field to map to.
    #[serde(
        rename = "incident_field",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub incident_field: Option<Option<String>>,
    /// The Jira field key.
    #[serde(rename = "jira_field_key")]
    pub jira_field_key: String,
    /// The type of the Jira field.
    #[serde(
        rename = "jira_field_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub jira_field_type: Option<Option<String>>,
    /// The direction of synchronization.
    #[serde(rename = "sync_direction")]
    pub sync_direction: String,
    /// Mapping of values between incident and Jira fields.
    #[serde(rename = "value_mapping")]
    pub value_mapping: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentJiraTemplateFieldConfiguration {
    pub fn new(
        jira_field_key: String,
        sync_direction: String,
    ) -> IncidentJiraTemplateFieldConfiguration {
        IncidentJiraTemplateFieldConfiguration {
            custom_outbound_value: None,
            incident_field: None,
            jira_field_key,
            jira_field_type: None,
            sync_direction,
            value_mapping: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_outbound_value(mut self, value: serde_json::Value) -> Self {
        self.custom_outbound_value = Some(value);
        self
    }

    pub fn incident_field(mut self, value: Option<String>) -> Self {
        self.incident_field = Some(value);
        self
    }

    pub fn jira_field_type(mut self, value: Option<String>) -> Self {
        self.jira_field_type = Some(value);
        self
    }

    pub fn value_mapping(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.value_mapping = Some(value);
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

impl<'de> Deserialize<'de> for IncidentJiraTemplateFieldConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentJiraTemplateFieldConfigurationVisitor;
        impl<'a> Visitor<'a> for IncidentJiraTemplateFieldConfigurationVisitor {
            type Value = IncidentJiraTemplateFieldConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_outbound_value: Option<serde_json::Value> = None;
                let mut incident_field: Option<Option<String>> = None;
                let mut jira_field_key: Option<String> = None;
                let mut jira_field_type: Option<Option<String>> = None;
                let mut sync_direction: Option<String> = None;
                let mut value_mapping: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_outbound_value" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_outbound_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_field" => {
                            incident_field =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_field_key" => {
                            jira_field_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_field_type" => {
                            jira_field_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_direction" => {
                            sync_direction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_mapping" => {
                            if v.is_null() {
                                continue;
                            }
                            value_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let jira_field_key =
                    jira_field_key.ok_or_else(|| M::Error::missing_field("jira_field_key"))?;
                let sync_direction =
                    sync_direction.ok_or_else(|| M::Error::missing_field("sync_direction"))?;

                let content = IncidentJiraTemplateFieldConfiguration {
                    custom_outbound_value,
                    incident_field,
                    jira_field_key,
                    jira_field_type,
                    sync_direction,
                    value_mapping,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentJiraTemplateFieldConfigurationVisitor)
    }
}

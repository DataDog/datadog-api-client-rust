// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident integration settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationIncident {
    /// Query for auto-escalation
    #[serde(rename = "auto_escalation_query")]
    pub auto_escalation_query: Option<String>,
    /// Default incident commander
    #[serde(rename = "default_incident_commander")]
    pub default_incident_commander: Option<String>,
    /// Whether incident integration is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "field_mappings")]
    pub field_mappings: Option<Vec<crate::datadogV2::model::IntegrationIncidentFieldMappingsItems>>,
    /// Incident type
    #[serde(rename = "incident_type")]
    pub incident_type: Option<String>,
    #[serde(rename = "severity_config")]
    pub severity_config: Option<crate::datadogV2::model::IntegrationIncidentSeverityConfig>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationIncident {
    pub fn new() -> IntegrationIncident {
        IntegrationIncident {
            auto_escalation_query: None,
            default_incident_commander: None,
            enabled: None,
            field_mappings: None,
            incident_type: None,
            severity_config: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_escalation_query(mut self, value: String) -> Self {
        self.auto_escalation_query = Some(value);
        self
    }

    pub fn default_incident_commander(mut self, value: String) -> Self {
        self.default_incident_commander = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn field_mappings(
        mut self,
        value: Vec<crate::datadogV2::model::IntegrationIncidentFieldMappingsItems>,
    ) -> Self {
        self.field_mappings = Some(value);
        self
    }

    pub fn incident_type(mut self, value: String) -> Self {
        self.incident_type = Some(value);
        self
    }

    pub fn severity_config(
        mut self,
        value: crate::datadogV2::model::IntegrationIncidentSeverityConfig,
    ) -> Self {
        self.severity_config = Some(value);
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

impl Default for IntegrationIncident {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationIncident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationIncidentVisitor;
        impl<'a> Visitor<'a> for IntegrationIncidentVisitor {
            type Value = IntegrationIncident;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_escalation_query: Option<String> = None;
                let mut default_incident_commander: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut field_mappings: Option<
                    Vec<crate::datadogV2::model::IntegrationIncidentFieldMappingsItems>,
                > = None;
                let mut incident_type: Option<String> = None;
                let mut severity_config: Option<
                    crate::datadogV2::model::IntegrationIncidentSeverityConfig,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_escalation_query" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_escalation_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_incident_commander" => {
                            if v.is_null() {
                                continue;
                            }
                            default_incident_commander =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_mappings" => {
                            if v.is_null() {
                                continue;
                            }
                            field_mappings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_type" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity_config" => {
                            if v.is_null() {
                                continue;
                            }
                            severity_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationIncident {
                    auto_escalation_query,
                    default_incident_commander,
                    enabled,
                    field_mappings,
                    incident_type,
                    severity_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationIncidentVisitor)
    }
}

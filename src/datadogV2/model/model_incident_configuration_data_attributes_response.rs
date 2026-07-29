// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident configuration in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentConfigurationDataAttributesResponse {
    /// Timestamp when the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether integrations are executed for this incident.
    #[serde(rename = "execute_integrations")]
    pub execute_integrations: Option<bool>,
    /// Whether notification rules are executed for this incident.
    #[serde(rename = "execute_notification_rules")]
    pub execute_notification_rules: Option<bool>,
    /// The incident identifier.
    #[serde(rename = "incident_id")]
    pub incident_id: String,
    /// Whether this incident is included in analytics.
    #[serde(rename = "include_in_analytics")]
    pub include_in_analytics: Option<bool>,
    /// Whether this incident is included in search results.
    #[serde(rename = "include_in_search")]
    pub include_in_search: Option<bool>,
    /// Timestamp when the configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentConfigurationDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        incident_id: String,
        modified_at: chrono::DateTime<chrono::Utc>,
    ) -> IncidentConfigurationDataAttributesResponse {
        IncidentConfigurationDataAttributesResponse {
            created_at,
            execute_integrations: None,
            execute_notification_rules: None,
            incident_id,
            include_in_analytics: None,
            include_in_search: None,
            modified_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn execute_integrations(mut self, value: bool) -> Self {
        self.execute_integrations = Some(value);
        self
    }

    pub fn execute_notification_rules(mut self, value: bool) -> Self {
        self.execute_notification_rules = Some(value);
        self
    }

    pub fn include_in_analytics(mut self, value: bool) -> Self {
        self.include_in_analytics = Some(value);
        self
    }

    pub fn include_in_search(mut self, value: bool) -> Self {
        self.include_in_search = Some(value);
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

impl<'de> Deserialize<'de> for IncidentConfigurationDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentConfigurationDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentConfigurationDataAttributesResponseVisitor {
            type Value = IncidentConfigurationDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut execute_integrations: Option<bool> = None;
                let mut execute_notification_rules: Option<bool> = None;
                let mut incident_id: Option<String> = None;
                let mut include_in_analytics: Option<bool> = None;
                let mut include_in_search: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execute_integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            execute_integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execute_notification_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            execute_notification_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_in_analytics" => {
                            if v.is_null() {
                                continue;
                            }
                            include_in_analytics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_in_search" => {
                            if v.is_null() {
                                continue;
                            }
                            include_in_search =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;

                let content = IncidentConfigurationDataAttributesResponse {
                    created_at,
                    execute_integrations,
                    execute_notification_rules,
                    incident_id,
                    include_in_analytics,
                    include_in_search,
                    modified_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentConfigurationDataAttributesResponseVisitor)
    }
}

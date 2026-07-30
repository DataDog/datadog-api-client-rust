// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an incident configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentConfigurationDataAttributesRequest {
    /// Whether to execute integrations for this incident.
    #[serde(rename = "execute_integrations")]
    pub execute_integrations: Option<bool>,
    /// Whether to execute notification rules for this incident.
    #[serde(rename = "execute_notification_rules")]
    pub execute_notification_rules: Option<bool>,
    /// Whether to include this incident in analytics.
    #[serde(rename = "include_in_analytics")]
    pub include_in_analytics: Option<bool>,
    /// Whether to include this incident in search results.
    #[serde(rename = "include_in_search")]
    pub include_in_search: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentConfigurationDataAttributesRequest {
    pub fn new() -> IncidentConfigurationDataAttributesRequest {
        IncidentConfigurationDataAttributesRequest {
            execute_integrations: None,
            execute_notification_rules: None,
            include_in_analytics: None,
            include_in_search: None,
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

impl Default for IncidentConfigurationDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentConfigurationDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentConfigurationDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentConfigurationDataAttributesRequestVisitor {
            type Value = IncidentConfigurationDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut execute_integrations: Option<bool> = None;
                let mut execute_notification_rules: Option<bool> = None;
                let mut include_in_analytics: Option<bool> = None;
                let mut include_in_search: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentConfigurationDataAttributesRequest {
                    execute_integrations,
                    execute_notification_rules,
                    include_in_analytics,
                    include_in_search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentConfigurationDataAttributesRequestVisitor)
    }
}
